use crate::ast::Ast;
use crate::builtins;

use std::collections::HashMap;

pub struct Environment<'a> {
    outer: Option<&'a Environment<'a>>,
    bindings: HashMap<String, Ast>,
}

impl<'a> Environment<'a> {
    pub fn with_binds<I: IntoIterator<Item = (String, Ast)>>(
        outer: &'a Environment<'a>,
        bindings: I,
    ) -> Self {
        Self {
            outer: Some(outer),
            bindings: bindings.into_iter().collect(),
        }
    }
    pub fn with_outer(outer: &'a Environment<'a>) -> Self {
        Self {
            outer: Some(outer),
            bindings: HashMap::new(),
        }
    }
    pub fn outer_new() -> Self {
        Self {
            outer: None,
            bindings: builtins::builtins_hashmap(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Ast> {
        let in_self = self.bindings.get(name).cloned();
        if in_self.is_some() {
            in_self
        } else if let Some(outer) = self.outer {
            outer.get(name)
        } else {
            None
        }
    }

    pub fn bind(&mut self, binding: String, value: Ast) {
        self.bindings.insert(binding, value);
    }
}
