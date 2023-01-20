use crate::ast::{Ast, LispAtom};
use crate::env::Environment;
use crate::error::LispError;

pub fn eval_expr(input: Ast, env: &mut Environment) -> Result<Ast, LispError> {
    Ok(match input {
        Ast::List(list) => eval_list(list, env)?,
        Ast::Atom(LispAtom::Symbol(symbol)) => eval_symbol(&symbol, env)?, // Symbols get looked up in environment
        Ast::Atom(_) => input,     // Other atoms return themselves
        Ast::Function(_) => input, // Functions return themselves
    })
}

pub fn eval_list(list: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    debug_assert!(list.len() > 0);
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

pub fn eval_symbol(symbol: &str, env: &mut Environment) -> Result<Ast, LispError> {
    // Look up symbol in environment
    env.get(symbol).ok_or(LispError::TypeError)
}
