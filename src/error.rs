//! Contains a single enum that represents all errors that can occur in the interpreter.

/// All the possible errors that can occur inside the interpreter.
#[derive(Debug)]
pub enum LispError {
    /// Error with input/output. Usually means a filesystem error occured.
    IOError,

    /// Error parsing an expression.
    ParseError,

    /// Type error.
    TypeError,

    /// Function called with incorrect number of arguments.
    BadArity,
}
