//! Contains a single enum that represents all errors that can occur in the interpreter.

use std::fmt::Display;

use colored::Colorize;

/// All the possible errors that can occur inside the interpreter.
#[derive(Debug)]
pub enum LispError {
    /// Error with input/output. Usually means a filesystem error occured.
    IOError,

    /// Error parsing an expression.
    ParseError(String),

    /// Constant or function not defined.
    Undefined(String),

    /// Type error.
    /// TODO: Add "expected" and "got"
    TypeError,

    /// Function called with incorrect number of arguments.
    BadArity,
}

impl Display for LispError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LispError::IOError => write!(f, "{}", "ERROR: IO error.".red()),
            LispError::ParseError(expr) => {
                write!(f, "{} {}", "ERROR: Could not parse expression:".red(), expr)
            }
            LispError::Undefined(ident) => {
                write!(f, "{} {}", "ERROR: Undefined identifier:".red(), ident)
            }
            LispError::TypeError => todo!(),
            LispError::BadArity => todo!(),
        }
    }
}
