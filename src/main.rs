//! # callisp - Callum's Lisp
//!
//! This is a Lisp interpreter. Doesn't do much yet.

#![warn(missing_docs)]

mod ast;
mod builtins;
mod env;
mod error;
mod eval;
mod parser;
mod repl;
mod special_forms;

fn main() {
    // If the user passed a filename, execute file
    // otherwise, start REPL
    repl::repl();
}
