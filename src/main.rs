//! # callisp - Callum's Lisp
//!
//! This is a Lisp interpreter. Doesn't do much yet.

#![warn(missing_docs)]

use std::{fs::read_to_string, path::PathBuf};

use structopt::StructOpt;

use ast::Ast;
use error::LispError;

use crate::env::Environment;

mod ast;
mod builtins;
mod env;
mod error;
mod eval;
mod parser;
mod repl;
mod special_forms;

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
    let mut res = Ast::List(vec![]);

    while let Ok((rest, expr)) = parser::parse_expr(to_parse) {
        to_parse = rest;
        res = eval::eval_expr(expr, env)?;
    }

    Ok(res)
}

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    if let Some(file) = opt.file {
        let mut env = Environment::outer_new();
        let _output = execute_file(file, &mut env).unwrap();
    } else {
        repl::repl();
    }
}
