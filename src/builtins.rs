use crate::ast::{Ast, FunctionArity, LispAtom, LispCallable};
use crate::env::Environment;
use crate::error::LispError;
use crate::{eval, parser};

use lazy_static::lazy_static;
use std::collections::HashMap;

/// A macro to make the next function more readable.
macro_rules! fn_box {
    ($func:ident) => {
        Ast::Function(Box::new($func))
    };
}

pub fn builtins_hashmap() -> HashMap<String, Ast> {
    HashMap::from([
        ("+".to_string(), fn_box!(LispAdd)),
        ("-".to_string(), fn_box!(LispSub)),
        ("*".to_string(), fn_box!(LispMul)),
        ("/".to_string(), fn_box!(LispDiv)),
        ("eval".to_string(), fn_box!(LispEval)),
        ("exit".to_string(), fn_box!(LispExit)),
        ("use".to_string(), fn_box!(LispUse)),
        ("putstr".to_string(), fn_box!(LispPutStr)),
        ("readline".to_string(), fn_box!(LispReadLine)),
        ("use".to_string(), fn_box!(LispUse)),
        ("equal?".to_string(), fn_box!(LispEqual)),
    ])
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
