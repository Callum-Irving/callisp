use crate::ast::Ast;
use crate::builtins;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Environment {
    bindings: Vec<HashMap<String, Ast>>,
}

impl Environment {
    pub fn with_binds(bindings: HashMap<String, Ast>) -> Self {
        Self {
            bindings: vec![bindings],
        }
    }

    pub fn outer_new() -> Self {
        Self {
            bindings: vec![builtins::builtins_hashmap()],
        }
    }

    pub fn new_scope(&mut self, bindings: HashMap<String, Ast>) {
        self.bindings.push(bindings);
    }

    // TODO: Return result
    pub fn pop_scope(&mut self) {
        self.bindings.pop();
    }

    pub fn get(&self, binding: &str) -> Option<Ast> {
        self.bindings
            .iter()
            .rev()
            .find(|map| map.contains_key(binding))
            .and_then(|map| map.get(binding).cloned())
    }

    pub fn bind(&mut self, binding: String, value: Ast) {
        self.bindings
            .last_mut()
            .expect("empty environment")
            .insert(binding, value);
    }
}

// pub struct Environment<'a> {
//     outer: Option<&'a Environment<'a>>,
//     bindings: HashMap<String, Ast>,
// }
//
// impl<'a> Environment<'a> {
//     pub fn with_binds<I: IntoIterator<Item = (String, Ast)>>(
//         outer: &'a Environment<'a>,
//         bindings: I,
//     ) -> Self {
//         Self {
//             outer: Some(outer),
//             bindings: bindings.into_iter().collect(),
//         }
//     }
//     pub fn with_outer(outer: &'a Environment<'a>) -> Self {
//         Self {
//             outer: Some(outer),
//             bindings: HashMap::new(),
//         }
//     }
//     pub fn outer_new() -> Self {
//         Self {
//             outer: None,
//             bindings: builtins::builtins_hashmap(),
//         }
//     }
//
//     pub fn get(&self, name: &str) -> Option<Ast> {
//         let in_self = self.bindings.get(name).cloned();
//         if in_self.is_some() {
//             in_self
//         } else if let Some(outer) = self.outer {
//             outer.get(name)
//         } else {
//             None
//         }
//     }
//
//     pub fn bind(&mut self, binding: String, value: Ast) {
//         self.bindings.insert(binding, value);
//     }
// }
//
