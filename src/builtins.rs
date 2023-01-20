use crate::ast::{Ast, FunctionArity, LispAtom, LispCallable};
use crate::env::Environment;
use crate::error::LispError;
use crate::eval;

use std::collections::HashMap;

pub fn builtins_hashmap() -> HashMap<String, Ast> {
    HashMap::from([
        ("+".to_string(), Ast::Function(Box::new(LispAdd))),
        ("-".to_string(), Ast::Function(Box::new(LispSub))),
        ("*".to_string(), Ast::Function(Box::new(LispMul))),
        ("/".to_string(), Ast::Function(Box::new(LispDiv))),
        ("eval".to_string(), Ast::Function(Box::new(LispEval))),
        ("exit".to_string(), Ast::Function(Box::new(LispExit))),
    ])
}

fn ast_to_num(ast: Ast) -> Result<f64, LispError> {
    match ast {
        Ast::Atom(LispAtom::Number(num)) => Ok(num),
        _ => Err(LispError::TypeError),
    }
}

fn take_first(items: Vec<Ast>) -> Result<Ast, LispError> {
    items.into_iter().next().ok_or(LispError::TypeError)
}

fn to_list_of_nums(args: Vec<Ast>) -> Result<Vec<f64>, LispError> {
    args.iter()
        .map(|ast| match ast {
            Ast::Atom(LispAtom::Number(num)) => Ok(*num),
            _ => Err(LispError::TypeError),
        })
        .collect::<Result<Vec<f64>, LispError>>()
}

const EXACTLY_1: FunctionArity = FunctionArity::Exactly(1);
const AT_LEAST_1: FunctionArity = FunctionArity::AtLeast(1);

#[derive(Debug, Clone)]
struct LispExit;

impl LispCallable for LispExit {
    fn arity(&self) -> &FunctionArity {
        &EXACTLY_1
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
        &EXACTLY_1
    }

    fn call(&self, args: Vec<Ast>, env: &mut crate::env::Environment) -> Result<Ast, LispError> {
        eval::eval_expr(take_first(args)?, env)
    }
}

#[derive(Debug, Clone)]
struct LispAdd;

impl LispCallable for LispAdd {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_1
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
        &AT_LEAST_1
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let difference = if args.len() > 1 {
            to_list_of_nums(args)?
                .into_iter()
                .reduce(|acc, num| acc - num)
                .ok_or(LispError::TypeError)?
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
        &AT_LEAST_1
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let product = to_list_of_nums(args)?
            .into_iter()
            .reduce(|acc, num| acc * num)
            .ok_or(LispError::TypeError)?;

        Ok(Ast::Atom(LispAtom::Number(product)))
    }
}

#[derive(Debug, Clone)]
struct LispDiv;

impl LispCallable for LispDiv {
    fn arity(&self) -> &FunctionArity {
        &AT_LEAST_1
    }

    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let quotient = if args.len() > 1 {
            to_list_of_nums(args)?
                .into_iter()
                .reduce(|acc, num| acc * num)
                .ok_or(LispError::TypeError)?
        } else {
            1.0 / take_first(args).and_then(ast_to_num)?
        };

        Ok(Ast::Atom(LispAtom::Number(quotient)))
    }
}
