use crate::ast::{Ast, LispAtom, LispCallable};
use crate::env::Environment;
use crate::error::LispError;

use std::collections::HashMap;

pub fn builtins_hashmap() -> HashMap<String, Ast> {
    HashMap::from([
        ("+".to_string(), Ast::Function(Box::new(LispAdd))),
        ("-".to_string(), Ast::Function(Box::new(LispSub))),
        ("*".to_string(), Ast::Function(Box::new(LispMul))),
        ("/".to_string(), Ast::Function(Box::new(LispDiv))),
    ])
}

fn to_list_of_nums(args: Vec<Ast>) -> Result<Vec<f64>, LispError> {
    args.iter()
        .map(|ast| match ast {
            Ast::Atom(LispAtom::Number(num)) => Ok(*num),
            _ => Err(LispError::TypeError),
        })
        .collect::<Result<Vec<f64>, LispError>>()
}

#[derive(Debug, Clone)]
struct LispAdd;

impl LispCallable for LispAdd {
    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let sum = to_list_of_nums(args)?.into_iter().sum();

        Ok(Ast::Atom(LispAtom::Number(sum)))
    }
}

#[derive(Debug, Clone)]
struct LispSub;

impl LispCallable for LispSub {
    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let difference = to_list_of_nums(args)?
            .into_iter()
            .reduce(|acc, num| acc - num)
            .ok_or(LispError::TypeError)?;

        Ok(Ast::Atom(LispAtom::Number(difference)))
    }
}

#[derive(Debug, Clone)]
struct LispMul;

impl LispCallable for LispMul {
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
    fn call(&self, args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
        let quotient = to_list_of_nums(args)?
            .into_iter()
            .reduce(|acc, num| acc * num)
            .ok_or(LispError::TypeError)?;

        Ok(Ast::Atom(LispAtom::Number(quotient)))
    }
}
