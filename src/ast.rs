use crate::env::{self, Environment};
use crate::error::LispError;
use crate::eval;
use dyn_clone::DynClone;

use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Ast {
    Atom(LispAtom),
    List(Vec<Ast>),
    Function(Box<dyn LispCallable>),
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
            Ast::Function(_) => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
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
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LispAtom {
    Symbol(String),
    String(String),
    Number(f64),
}

impl Display for LispAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(num) => write!(f, "{}", num),
        }
    }
}

impl Clone for Box<dyn LispCallable> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

#[derive(Debug, Clone)]
pub enum FunctionArity {
    AtLeast(usize),
    Exactly(usize),
}

impl FunctionArity {
    pub fn check_arity(&self, num_args: usize) -> Result<(), LispError> {
        match self {
            Self::AtLeast(num_params) => {
                if num_args >= *num_params {
                    Ok(())
                } else {
                    Err(LispError::TypeError)
                }
            }
            Self::Exactly(num_params) => {
                if num_args == *num_params {
                    Ok(())
                } else {
                    Err(LispError::TypeError)
                }
            }
        }
    }
}

pub trait LispCallable: Debug + DynClone {
    fn arity(&self) -> &FunctionArity;

    fn call(&self, args: Vec<Ast>, env: &mut env::Environment) -> Result<Ast, LispError>;
}

#[derive(Debug, Clone)]
pub struct LispLambda {
    arity: FunctionArity,
    bindings: Vec<String>,
    body: Ast,
}

impl LispLambda {
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

    fn call(&self, args: Vec<Ast>, env: &mut env::Environment) -> Result<Ast, LispError> {
        // Create bindings
        let mut env = Environment::with_binds(env, self.bindings.iter().cloned().zip(args));

        // Evaluate in new environment
        eval::eval_expr(self.body.clone(), &mut env)
    }
}
