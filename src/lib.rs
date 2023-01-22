//! # callisp - Callum's Lisp
//!
//! This is a Lisp interpreter, written with the intention of being used in WASM.

#![warn(missing_docs)]

pub mod ast;
pub mod builtins;
pub mod env;
pub mod error;
pub mod eval;
pub mod parser;
pub mod special_forms;

use wasm_bindgen::prelude::*;

/// Parses a string to an expression then evaluates that expression and returns the string
/// representation of the result.
#[wasm_bindgen]
pub fn parse_eval_print(input: String, env: &mut env::Environment) -> String {
    let expr = parser::parse_expr(&input).unwrap().1;
    let res = eval::eval_expr(expr, env).unwrap();
    format!("{}", res)
}
