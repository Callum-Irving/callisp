pub mod ast;
pub mod builtins;
pub mod env;
pub mod error;
pub mod eval;
pub mod parser;
pub mod special_forms;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_eval_print(input: String, env: &mut env::Environment) -> String {
    let expr = parser::parse_expr(&input).unwrap().1;
    let res = eval::eval_expr(expr, env).unwrap();
    format!("{}", res)
}
