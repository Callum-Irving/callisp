use std::io::{self, Write};

use crate::ast::Ast;
use crate::env::Environment;
use crate::error::LispError;
use crate::eval::eval_expr;
use crate::parser::parse_complete_expr;

fn read() -> Result<Ast, LispError> {
    print!("callisp> ");
    io::stdout().flush().map_err(|_| LispError::IOError)?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .map_err(|_| LispError::IOError)?;
    let buf = buf.trim_end().to_string();

    // TODO: Process input
    let expr = parse_complete_expr(&buf)
        .map_err(|_| LispError::ParseError(buf.to_string()))?
        .1;

    Ok(expr)
}

fn eval(input: Ast, env: &mut Environment) -> Result<Ast, LispError> {
    eval_expr(input, env)
}

fn print(input: Ast) {
    if !matches!(input, Ast::Unspecified) {
        println!("{}", input);
    }
}

pub fn repl() {
    let mut env = Environment::outer_new();
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
