//! Contains all the built-in special forms such as def, lambda, etc.

use std::collections::HashMap;

use crate::ast::{Ast, LispAtom, LispLambda};
use crate::env::Environment;
use crate::error::LispError;
use crate::eval;

use lazy_static::lazy_static;

type SpecialFormFunction = fn(Vec<Ast>, &mut Environment) -> Result<Ast, LispError>;

lazy_static! {
    pub(crate) static ref SPECIAL_FORMS: HashMap<&'static str, SpecialFormFunction> = {
        let mut map: HashMap<&'static str, SpecialFormFunction> = HashMap::new();
        map.insert("λ", lambda);
        map.insert("lambda", lambda);
        map.insert("def", define);
        map.insert("if", lisp_if);
        map.insert("quote", quote);
        map
    };
}

#[inline(always)]
pub(crate) fn eval_special_form(
    input: Vec<Ast>,
    env: &mut Environment,
    special_form: &SpecialFormFunction,
) -> Result<Ast, LispError> {
    let args = input.into_iter().skip(1).collect();
    special_form(args, env)
}

/// If statement.
pub fn lisp_if(args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    let mut args = args.into_iter();

    let condition = args.next().ok_or(LispError::BadArity)?;
    let condition = eval::eval_expr(condition, env)?;

    if condition != Ast::Atom(LispAtom::Bool(false)) {
        // Evaluate true block
        eval::eval_expr(args.next().ok_or(LispError::BadArity)?, env)
    } else {
        // Evaluate else block
        eval::eval_expr(args.nth(1).ok_or(LispError::BadArity)?, env)
    }
}

/// Create a binding in the current environment.
///
/// Example:
/// `(define x 3)` binds x to 3. Now the expression `x` returns 3.
pub fn define(args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    let mut args = args.into_iter();

    let Some(Ast::Atom(LispAtom::Symbol(binding))) = args.next() else {
        return Err(LispError::TypeError);
    };

    let value = eval::eval_expr(args.next().ok_or(LispError::BadArity)?, env)?;

    env.bind(binding, value);

    Ok(Ast::Unspecified)
}

/// Create a lambda function.
///
/// Example:
/// `(lambda (x) (+ x 1))` creates a function that adds 1 to x.
pub fn lambda(args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
    let mut args = args.into_iter();

    // convert Vec<Ast> to Vec<String>
    let bindings: Vec<_> = if let Some(Ast::List(bindings)) = args.next() {
        bindings
            .iter()
            .map(|ast| match ast {
                Ast::Atom(LispAtom::Symbol(symbol)) => Ok(symbol.clone()),
                _ => Err(LispError::TypeError),
            })
            .collect::<Result<_, LispError>>()?
    } else {
        return Err(LispError::TypeError);
    };

    let body = args.next().ok_or(LispError::BadArity)?;

    let lambda = LispLambda::new(bindings.len(), bindings, body);

    Ok(Ast::Function(Box::new(lambda)))
}

/// Quote a lisp value.
///
/// Example:
/// `(define a 3)`
/// `(quote a) => a` returns a instead of returning the defined value of a.
pub fn quote(args: Vec<Ast>, _env: &mut Environment) -> Result<Ast, LispError> {
    if args.len() != 1 {
        return Err(LispError::TypeError);
    }

    let arg = args.into_iter().next().ok_or(LispError::BadArity)?;

    Ok(arg)
}
