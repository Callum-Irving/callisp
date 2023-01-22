//! Contains the struct [Environment] that stores current bindings for the interpreter.

use crate::ast::Ast;
use crate::builtins;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

/// The environment that expressions are evaluated in.
#[wasm_bindgen]
pub struct Environment {
    bindings: Vec<HashMap<String, Ast>>,
}

impl Environment {
    /// Create an environment with the specified bindings at the outermost scope.
    pub fn with_binds(bindings: HashMap<String, Ast>) -> Self {
        Self {
            bindings: vec![bindings],
        }
    }

    /// Create an environment with the bindings for all builtins.
    pub fn outer_new() -> Self {
        Self {
            bindings: vec![builtins::builtins_hashmap()],
        }
    }

    /// Add an empty scope to the environment.
    pub fn new_scope(&mut self, bindings: HashMap<String, Ast>) {
        self.bindings.push(bindings);
    }

    /// Remove the top scope from the environment.
    pub fn pop_scope(&mut self) {
        self.bindings.pop();
    }

    /// Get the Ast matching a string stored in the bindings of the environment.
    pub fn get(&self, binding: &str) -> Option<Ast> {
        self.bindings
            .iter()
            .rev()
            .find(|map| map.contains_key(binding))
            .and_then(|map| map.get(binding).cloned())
    }

    /// Set a new binding in the environment. Will overwrite current binding if one exists.
    ///
    /// TODO: Don't overwrite binding.
    pub fn bind(&mut self, binding: String, value: Ast) {
        self.bindings
            .last_mut()
            .expect("empty environment")
            .insert(binding, value);
    }
}
