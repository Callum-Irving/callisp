//! Contains the functions used to evaluate an AST.

use crate::ast::{Ast, LispAtom};
use crate::env::Environment;
use crate::error::LispError;
use crate::special_forms::{eval_special_form, SPECIAL_FORMS};

/// Evaluate a lisp expression.
pub fn eval_expr(input: Ast, env: &mut Environment) -> Result<Ast, LispError> {
    match input {
        Ast::List(list) => match list.get(0) {
            Some(Ast::Atom(LispAtom::Symbol(symbol))) => {
                if let Some(special_form) = SPECIAL_FORMS.get(symbol.as_str()) {
                    eval_special_form(list, env, special_form)
                } else {
                    eval_list(list, env)
                }
            }
            _ => eval_list(list, env),
        },
        Ast::Atom(LispAtom::Symbol(symbol)) => eval_symbol(&symbol, env), // Symbols get looked up in environment
        _ => Ok(input), // Atoms and functions return themselves
    }
}

fn eval_list(list: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    // eval first item of list
    // should be Ast::Function
    // call the function on rest(list)
    let mut list = list.into_iter();

    let func = list
        .next()
        .ok_or(LispError::TypeError)
        .and_then(|ast| eval_expr(ast, env))?;

    let args: Vec<Ast> = list
        .map(|ast| eval_expr(ast, env))
        .collect::<Result<_, LispError>>()?;

    if let Ast::Function(func) = func {
        func.arity().check_arity(args.len())?;
        func.call(args, env)
    } else {
        Err(LispError::TypeError)
    }
}

fn eval_symbol(symbol: &str, env: &mut Environment) -> Result<Ast, LispError> {
    // Look up symbol in environment
    env.get(symbol)
        .ok_or(LispError::Undefined(symbol.to_string()))
}
