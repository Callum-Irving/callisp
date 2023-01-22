//! Contains a single enum that represents all errors that can occur in the interpreter.

/// All the possible errors that can occur inside the interpreter.
#[derive(Debug)]
pub enum LispError {
    /// Error with input/output. Usually means a filesystem error occured.
    IO,

    /// Error parsing an expression.
    Parse,

    /// Type error.
    Type,
}
