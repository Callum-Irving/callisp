use std::io::{self, Write};

// mod ast;
// mod builtins;
// mod env;
// mod error;
// mod eval;
// mod parser;
//
// use ast::Ast;
// use error::LispError;

use callisp::ast::Ast;
use callisp::env;
use callisp::error::LispError;
use callisp::eval;
use callisp::parser;

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
    print!("callisp> ");
    io::stdout().flush().map_err(|_| LispError::IOError)?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .map_err(|_| LispError::IOError)?;
    let buf = buf.trim_end().to_string();

    // TODO: Process input
    let expr = parser::parse_complete_expr(&buf)
        .map_err(|_| LispError::ParseError(buf.to_string()))?
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
    if !matches!(input, Ast::Unspecified) {
        println!("{}", input);
    }
}

fn main() {
    let mut env = env::Environment::outer_new();
    loop {
        let input = match read() {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        let result = match eval(input, &mut env) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        print(result);
    }
}
