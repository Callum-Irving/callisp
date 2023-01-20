use std::io;

mod ast;
mod builtins;
mod env;
mod error;
mod eval;
mod parser;

use ast::{Ast, FunctionArity, LispAtom, LispLambda};
use error::LispError;

// Evaluating:
//
// If literal:
// Evaluates to itself
//
// If list:
// Look up first item in list in environment
// If function, run function
// If special form, run special form

fn read() -> Result<Ast, LispError> {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .map_err(|_| LispError::IOError)?;
    let buf = buf.trim_end().to_string();

    // TODO: Process input
    let expr = parser::parse_expr(&buf)
        .map_err(|_| LispError::ParseError)?
        .1;

    Ok(expr)
}

fn eval(input: Ast, env: &mut env::Environment) -> Result<Ast, LispError> {
    // input

    // Special forms:
    // - cond
    // - lambda
    // - define (needs to be at top level?)

    eval::eval_expr(input, env)
}

fn print(input: Ast) {
    println!("{}", input);
}

fn main() {
    let mut env = env::Environment::outer_new();
    loop {
        let input = read().unwrap();
        let result = eval(input, &mut env).unwrap();
        print(result);
    }
}
