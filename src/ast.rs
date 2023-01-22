//! The AST module contains structs and enums for the abstract syntax tree.

use crate::env::Environment;
use crate::error::LispError;
use crate::eval;
use dyn_clone::DynClone;

use std::fmt::{Debug, Display};

/// Stores an expression.
#[derive(Debug, Clone)]
pub enum Ast {
    /// An atom, such as a number, string, or symbol.
    Atom(LispAtom),

    /// A list created using ().
    List(Vec<Ast>),

    /// A callable function.
    Function(Box<dyn LispCallable>),

    /// Basically a none type.
    Unspecified,
}

impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Ast::Atom(val) => match other {
                Ast::Atom(other) => val == other,
                _ => false,
            },
            Ast::List(items) => match other {
                Ast::List(other) => items == other,
                _ => false,
            },
            // TODO: Maybe two functions are equal if they have the same body?
            Ast::Function(_) => false,
            Ast::Unspecified => false,
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{}", atom),
            Self::List(list) => {
                write!(f, "(")?;

                // Display a space-separated list of inner items
                let mut list = list.iter();
                if let Some(ast) = list.next() {
                    write!(f, "{}", ast)?;

                    for ast in list {
                        write!(f, " {}", ast)?;
                    }
                }

                write!(f, ")")
            }
            Self::Function(_) => write!(f, "<function>"),
            Self::Unspecified => Ok(()), // unspecified doesn't display anything
        }
    }
}

/// Lisp atom.
#[derive(Clone, Debug, PartialEq)]
pub enum LispAtom {
    /// A lisp symbol.
    Symbol(String),

    /// A lisp string. Stored as a Rust string which means it can store any unicode character.
    String(String),

    /// A lisp boolean.
    Bool(bool),

    /// A lisp number. Currently this is a double precision float.
    Number(f64),
}

impl Display for LispAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Number(num) => write!(f, "{}", num),
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl Clone for Box<dyn LispCallable> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

/// A struct representing the possible values of arity a function can have. Arity is just the
/// number of arguments a function takes.
#[derive(Debug, Clone)]
pub enum FunctionArity {
    /// The function can be called with a number of arguments greater than or equal to the
    /// contained value.
    AtLeast(usize),

    /// The function can be called with only the specified amount of arguments.
    Exactly(usize),

    /// The function can be called with any of the amounts of arguments stored in the vector.
    Multi(Vec<usize>),
}

impl FunctionArity {
    /// Check is the number of arguments passed to a function matches that functions arity.
    pub fn check_arity(&self, num_args: usize) -> Result<(), LispError> {
        match self {
            Self::AtLeast(num_params) => {
                if num_args >= *num_params {
                    Ok(())
                } else {
                    Err(LispError::Type)
                }
            }
            Self::Exactly(num_params) => {
                if num_args == *num_params {
                    Ok(())
                } else {
                    Err(LispError::Type)
                }
            }
            Self::Multi(options) => {
                if options.contains(&num_args) {
                    Ok(())
                } else {
                    Err(LispError::Type)
                }
            }
        }
    }
}

/// Trait used to define Lisp functions.
pub trait LispCallable: Debug + DynClone {
    /// Return the arity of the function.
    fn arity(&self) -> &FunctionArity;

    /// Call the function and return the result.
    fn call(&self, args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError>;
}

/// Function created using `lambda`.
#[derive(Debug, Clone)]
pub struct LispLambda {
    arity: FunctionArity,
    bindings: Vec<String>,
    body: Ast,
}

impl LispLambda {
    /// Create a new lambda function with specified arity, bindings, and body.
    pub fn new(arity: FunctionArity, bindings: Vec<String>, body: Ast) -> Self {
        Self {
            arity,
            bindings,
            body,
        }
    }
}

impl LispCallable for LispLambda {
    fn arity(&self) -> &FunctionArity {
        &self.arity
    }

    fn call(&self, args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
        // Create bindings
        env.new_scope(self.bindings.iter().cloned().zip(args).collect());

        // Evaluate in new environment
        let res = eval::eval_expr(self.body.clone(), env);

        env.pop_scope();

        res
    }
}
