use crate::ast::{Ast, FunctionArity, LispAtom, LispLambda};
use crate::env::Environment;
use crate::error::LispError;

pub fn eval_expr(input: Ast, env: &mut Environment) -> Result<Ast, LispError> {
    Ok(match input {
        Ast::List(list) => match list.get(0) {
            Some(Ast::Atom(LispAtom::Symbol(symbol))) => match symbol.as_str() {
                "lambda" => {
                    // items[1] = bindings
                    // convert Vec<Ast> to Vec<String>
                    let bindings: Vec<_> = if let Ast::List(bindings) = &list[1] {
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

                    // items[2] = body
                    let body = list[2].clone();

                    let lambda =
                        LispLambda::new(FunctionArity::Exactly(bindings.len()), bindings, body);

                    Ast::Function(Box::new(lambda))
                }
                "define" => {
                    let Some(Ast::Atom(LispAtom::Symbol(binding))) = list.get(1).cloned() else {
                        return Err(LispError::Type);
                    };

                    let value = eval_expr(list[2].clone(), env)?;

                    env.bind(binding, value.clone());

                    value
                }
                _ => eval_list(list, env)?,
            },
            _ => eval_list(list, env)?,
        },
        Ast::Atom(LispAtom::Symbol(symbol)) => eval_symbol(&symbol, env)?, // Symbols get looked up in environment
        Ast::Atom(_) => input,     // Other atoms return themselves
        Ast::Function(_) => input, // Functions return themselves
    })
}

pub fn eval_list(list: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
    // eval first item of list
    // should be Ast::Function
    // call the function on rest(list)
    let mut list = list.into_iter();

    let func = list
        .next()
        .ok_or(LispError::Type)
        .and_then(|ast| eval_expr(ast, env))?;

    let args: Vec<Ast> = list
        .map(|ast| eval_expr(ast, env))
        .collect::<Result<_, LispError>>()?;

    if let Ast::Function(func) = func {
        func.arity().check_arity(args.len())?;
        func.call(args, env)
    } else {
        Err(LispError::Type)
    }
}

pub fn eval_symbol(symbol: &str, env: &mut Environment) -> Result<Ast, LispError> {
    // Look up symbol in environment
    env.get(symbol).ok_or(LispError::Type)
}
