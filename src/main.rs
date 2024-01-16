//! # callisp - Callum's Lisp
//!
//! This is a Lisp interpreter. Doesn't do much yet.

#![warn(missing_docs)]

use std::fs::read_to_string;

use ast::Ast;
use error::LispError;

mod ast;
mod builtins;
mod env;
mod error;
mod eval;
mod parser;
mod repl;
mod special_forms;

fn execute_file(filename: &str, env: &mut env::Environment) -> Result<Ast, LispError> {
    let contents = read_to_string(filename).map_err(|_| LispError::IOError)?;
    let mut to_parse = contents.as_str();
    let mut res = Ast::List(vec![]);

    while let Ok((rest, expr)) = parser::parse_expr(to_parse) {
        to_parse = rest;
        res = eval::eval_expr(expr, env)?;
    }

    Ok(res)
}

fn main() {
    // If the user passed a filename, execute file
    // otherwise, start REPL
    repl::repl();
}
