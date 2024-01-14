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

    /// A type (like int or float).
    Type(LispType),

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
            Ast::Type(typ) => match other {
                Ast::Type(other) => typ == other,
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
            Self::Type(typ) => write!(f, "{}", typ),
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

    /// An integer.
    Int(i64),

    /// A floating point number.
    Float(f64),
}

impl Display for LispAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Int(n) => write!(f, "{}", n),
            Self::Float(n) => write!(f, "{}", n), // TODO: Is there a better way of formatting floats?
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
                    Err(LispError::BadArity)
                }
            }
            Self::Exactly(num_params) => {
                if num_args == *num_params {
                    Ok(())
                } else {
                    Err(LispError::BadArity)
                }
            }
            Self::Multi(options) => {
                if options.contains(&num_args) {
                    Ok(())
                } else {
                    Err(LispError::BadArity)
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

/// A Lisp type.
/// TODO: Add structs (user-defined types).
#[derive(Debug, Clone, PartialEq)]
pub enum LispType {
    /// An integer.
    Int,

    /// A floating point number.
    Float,

    /// A string.
    String,

    /// A boolean value.
    Bool,

    /// A list.
    List,

    /// A function.
    Function,

    /// A type. This can be a bit confusing.
    Type,

    /// A symbol.
    Symbol,

    /// An unspecified type.
    Unspecified,
}

impl Display for LispType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "builtin type int"),
            Self::Float => write!(f, "builtin type float"),
            Self::String => write!(f, "builtin type string"),
            Self::Bool => write!(f, "builtin type bool"),
            Self::List => write!(f, "list"),
            Self::Function => write!(f, "function"),
            Self::Type => write!(f, "type"),
            Self::Symbol => write!(f, "symbol"),
            Self::Unspecified => write!(f, "unspecified"),
        }
    }
}

impl From<&Ast> for LispType {
    fn from(value: &Ast) -> Self {
        match value {
            Ast::Atom(atom) => match atom {
                LispAtom::Symbol(_) => Self::Symbol,
                LispAtom::Int(_) => Self::Int,
                LispAtom::Float(_) => Self::Float,
                LispAtom::String(_) => Self::String,
                LispAtom::Bool(_) => Self::Bool,
            },
            Ast::List(_) => Self::List,
            Ast::Function(_) => Self::Function,
            Ast::Type(_) => Self::Type,
            Ast::Unspecified => Self::Unspecified,
        }
    }
}
