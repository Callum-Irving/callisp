//! Contains all the built-in functions for callisp.

use crate::ast::{Ast, FunctionArity, LispAtom, LispCallable};
use crate::env::Environment;
use crate::error::LispError;
use crate::{eval, parser};

use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! fn_map {
    ($($name:literal => $func:ident),+ ,) => {
        {
            let mut map: HashMap<String, Ast> = HashMap::new();
            $(
                map.insert($name.to_string(), Ast::Function(Box::new($func)));
            )+
            map
        }
    };
}

pub(crate) fn builtins_hashmap() -> HashMap<String, Ast> {
    fn_map! {
        "+" => LispAdd,
        "-" => LispSub,
        "*" => LispMul,
        "/" => LispDiv,
        "eval" => LispEval,
        "exit" => LispExit,
        "use" => LispUse,
        "putstr" => LispPutStr,
        "readline" => LispReadLine,
        "equal?" => LispEqual,
        ">" => LispGT,
        ">=" => LispGE,
        "<" => LispLT,
        "<=" => LispLE,
        "list" => LispList,
        "list?" => LispIsList,
        "empty?" => LispIsEmpty,
        "count" => LispCount,
    }
}

fn ast_to_num(ast: Ast) -> Result<f64, LispError> {
    match ast {
        Ast::Atom(LispAtom::Number(num)) => Ok(num),
        _ => Err(LispError::Type),
    }
}

fn ast_to_string(ast: Ast) -> Result<String, LispError> {
    match ast {
        Ast::Atom(LispAtom::String(string)) => Ok(string),
        _ => Err(LispError::Type),
    }
}

fn take_first(items: Vec<Ast>) -> Result<Ast, LispError> {
    items.into_iter().next().ok_or(LispError::Type)
}

fn get_first(items: &[Ast]) -> Result<&Ast, LispError> {
    items.get(0).ok_or(LispError::Type)
}

fn to_list_of_nums(args: Vec<Ast>) -> Result<Vec<f64>, LispError> {
    args.iter()
        .map(|ast| match ast {
            Ast::Atom(LispAtom::Number(num)) => Ok(*num),
            _ => Err(LispError::Type),
        })
        .collect::<Result<Vec<f64>, LispError>>()
}

lazy_static! {
    static ref ONE_OR_ZERO: FunctionArity = FunctionArity::Multi(vec![0, 1]);
}
const EXACTLY_ZERO: FunctionArity = FunctionArity::Exactly(0);
const EXACTLY_ONE: FunctionArity = FunctionArity::Exactly(1);
const AT_LEAST_ZERO: FunctionArity = FunctionArity::AtLeast(0);
const AT_LEAST_ONE: FunctionArity = FunctionArity::AtLeast(1);
const AT_LEAST_TWO: FunctionArity = FunctionArity::AtLeast(2);

#[derive(Debug, Clone)]
struct LispExit;

impl LispCallable for LispExit {
    fn arity(&self) -> &FunctionArity {
        &ONE_OR_ZERO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let code = match args.into_iter().next() {
            Some(code) => ast_to_num(code)? as i32,
            None => 0,
        };

        std::process::exit(code);
    }
}

#[derive(Debug, Clone)]
struct LispEval;

impl LispCallable for LispEval {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, env: &mut crate::env::Environment) -> Result<Ast, LispError> {
        eval::eval_expr(take_first(args)?, env)
    }
}

#[derive(Debug, Clone)]
struct LispAdd;

impl LispCallable for LispAdd {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let sum = if args.len() > 1 {
            to_list_of_nums(args)?.into_iter().sum()
        } else {
            take_first(args).and_then(ast_to_num)?
        };

        Ok(Ast::Atom(LispAtom::Number(sum)))
    }
}

#[derive(Debug, Clone)]
struct LispSub;

impl LispCallable for LispSub {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let difference = if args.len() > 1 {
            to_list_of_nums(args)?
                .into_iter()
                .reduce(|acc, num| acc - num)
                .ok_or(LispError::Type)?
        } else {
            -1.0 * take_first(args).and_then(ast_to_num)?
        };

        Ok(Ast::Atom(LispAtom::Number(difference)))
    }
}

#[derive(Debug, Clone)]
struct LispMul;

impl LispCallable for LispMul {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let product = to_list_of_nums(args)?
            .into_iter()
            .reduce(|acc, num| acc * num)
            .ok_or(LispError::Type)?;

        Ok(Ast::Atom(LispAtom::Number(product)))
    }
}

#[derive(Debug, Clone)]
struct LispDiv;

impl LispCallable for LispDiv {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let quotient = if args.len() > 1 {
            to_list_of_nums(args)?
                .into_iter()
                .reduce(|acc, num| acc / num)
                .ok_or(LispError::Type)?
        } else {
            1.0 / take_first(args).and_then(ast_to_num)?
        };

        Ok(Ast::Atom(LispAtom::Number(quotient)))
    }
}

#[derive(Debug, Clone)]
struct LispUse;

impl LispCallable for LispUse {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
        let file = take_first(args).and_then(ast_to_string)?;
        // convert file to string
        let contents = std::fs::read_to_string(file).map_err(|_| LispError::IO)?;
        let mut to_parse = contents.as_str();
        let mut res = Ast::List(vec![]);

        while let Ok((rest, expr)) = parser::parse_expr(to_parse) {
            to_parse = rest;
            res = eval::eval_expr(expr, env)?;
        }

        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct LispPutStr;

impl LispCallable for LispPutStr {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let string = take_first(args).and_then(ast_to_string)?;
        println!("{}", string);
        Ok(Ast::Unspecified)
    }
}

#[derive(Debug, Clone)]
struct LispReadLine;

impl LispCallable for LispReadLine {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ZERO
    }

    fn call(&self, _args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|_| LispError::IO)?;
        // TODO: This might break compatibility with windows
        buf.pop(); // Remove trailing '\n'
        Ok(Ast::Atom(LispAtom::String(buf)))
    }
}

#[derive(Debug, Clone)]
struct LispEqual;

impl LispCallable for LispEqual {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_TWO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let mut iter = args.into_iter();
        let first = iter.next().ok_or(LispError::Type)?;
        for arg in iter {
            if arg != first {
                return Ok(Ast::Atom(LispAtom::Bool(false)));
            }
        }

        Ok(Ast::Atom(LispAtom::Bool(true)))
    }
}

#[derive(Debug, Clone)]
struct LispGT;

impl LispCallable for LispGT {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_TWO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let nums = to_list_of_nums(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] > nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    }
}

#[derive(Debug, Clone)]
struct LispGE;

impl LispCallable for LispGE {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_TWO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let nums = to_list_of_nums(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] >= nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    }
}

#[derive(Debug, Clone)]
struct LispLT;

impl LispCallable for LispLT {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_TWO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let nums = to_list_of_nums(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] < nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    }
}

#[derive(Debug, Clone)]
struct LispLE;

impl LispCallable for LispLE {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_TWO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let nums = to_list_of_nums(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] <= nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    }
}

#[derive(Debug, Clone)]
struct LispList;

impl LispCallable for LispList {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_ZERO
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        Ok(Ast::List(args))
    }
}

#[derive(Debug, Clone)]
struct LispIsList;

impl LispCallable for LispIsList {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let is_list = matches!(get_first(&args)?, Ast::List(_));
        Ok(Ast::Atom(LispAtom::Bool(is_list)))
    }
}

#[derive(Debug, Clone)]
struct LispIsEmpty;

impl LispCallable for LispIsEmpty {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let arg = get_first(&args)?;
        let is_empty = match arg {
            Ast::List(items) => !items.is_empty(),
            _ => false,
        };

        Ok(Ast::Atom(LispAtom::Bool(is_empty)))
    }
}

#[derive(Debug, Clone)]
struct LispCount;

impl LispCallable for LispCount {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_ONE
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let arg = get_first(&args)?;
        let length = match arg {
            Ast::List(items) => items.len(),
            _ => return Err(LispError::Type),
        };

        Ok(Ast::Atom(LispAtom::Number(length as f64)))
    }
}
