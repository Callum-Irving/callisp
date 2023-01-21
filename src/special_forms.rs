use std::collections::HashMap;

use crate::ast::{Ast, FunctionArity, LispAtom, LispLambda};
use crate::env::Environment;
use crate::error::LispError;
use crate::eval;

use lazy_static::lazy_static;

type SpecialFormFunction = fn(Vec<Ast>, &mut Environment) -> Result<Ast, LispError>;

lazy_static! {
    pub static ref SPECIAL_FORMS: HashMap<&'static str, SpecialFormFunction> = {
        let mut map: HashMap<&'static str, SpecialFormFunction> = HashMap::new();
        map.insert("lambda", lambda);
        map.insert("define", define);
        map
    };
}

#[inline(always)]
pub fn eval_special_form(
    input: Vec<Ast>,
    env: &mut Environment,
    special_form: &SpecialFormFunction,
) -> Result<Ast, LispError> {
    let args = input.into_iter().skip(1).collect();
    special_form(args, env)
}

/// Create a binding in the current environment.
///
/// Example:
/// `(define x 3)` binds x to 3. Now the expression `x` returns 3.
pub fn define(args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    let mut args = args.into_iter();

    let Some(Ast::Atom(LispAtom::Symbol(binding))) = args.next() else {
        return Err(LispError::Type);
    };

    let value = eval::eval_expr(args.next().ok_or(LispError::Type)?, env)?;

    env.bind(binding, value.clone());

    Ok(value)
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
                _ => Err(LispError::Type),
            })
            .collect::<Result<_, LispError>>()?
    } else {
        return Err(LispError::Type);
    };

    let body = args.next().ok_or(LispError::Type)?;

    let lambda = LispLambda::new(FunctionArity::Exactly(bindings.len()), bindings, body);

    Ok(Ast::Function(Box::new(lambda)))
}
