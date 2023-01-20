use crate::ast::Ast;
use crate::builtins;

use std::collections::HashMap;

pub struct Environment<'a> {
    outer: Option<&'a Environment<'a>>,
    bindings: HashMap<String, Ast>,
}

impl<'a> Environment<'a> {
    pub fn outer_new() -> Self {
        Self {
            outer: None,
            bindings: builtins::builtins_hashmap(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Ast> {
        self.bindings.get(name).map(|ast| ast.clone())
    }
}
