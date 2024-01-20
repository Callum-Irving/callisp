//! # callisp - Callum's Lisp
//!
//! This is a Lisp interpreter. Doesn't do much yet.

#![warn(missing_docs)]

use std::{fs::read_to_string, path::PathBuf};

use structopt::StructOpt;

use ast::Ast;
use env::Environment;
use error::LispError;

mod ast;
mod builtins;
mod compiler;
mod env;
mod error;
mod eval;
mod lexer;
mod parser;
mod repl;
mod special_forms;
mod vm;

#[derive(Debug, StructOpt)]
#[structopt(name = "callisp", about = "Simple Lisp interpreter.")]
struct Opt {
    /// Input file. If left empty, will start REPL instead.
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn execute_file(filename: PathBuf, env: &mut env::Environment) -> Result<Ast, LispError> {
    let contents = read_to_string(filename).map_err(|_| LispError::IOError)?;
    let mut to_parse = contents.as_str();
    let mut exprs = vec![];

    // Parse whole file
    while let Ok((rest, expr)) = parser::parse_expr(to_parse) {
        to_parse = rest;
        exprs.push(expr);
    }

    if !to_parse.is_empty() {
        return Err(LispError::ParseError(to_parse.to_string()));
    }

    for expr in exprs {
        eval::eval_expr(expr, env)?;
    }

    Ok(Ast::Unspecified)
}

fn main() {
    let opt = Opt::from_args();

    if let Some(file) = opt.file {
        let mut env = Environment::outer_new();
        if let Err(e) = execute_file(file, &mut env) {
            eprintln!("{}", e);
        }
    } else {
        repl::repl();
    }
}
