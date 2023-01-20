use crate::env;
use crate::error::LispError;
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
            Ast::Function(func) => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        todo!("implement not equal for ast")
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("implement display for ast")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LispAtom {
    Symbol(String),
    String(String),
    Number(f64),
}

impl Clone for Box<dyn LispCallable> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

pub trait LispCallable: Debug + DynClone {
    fn call(&self, args: Vec<Ast>, env: &mut env::Environment) -> Result<Ast, LispError>;
}
