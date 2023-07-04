use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{builtin_function::BuiltinFunction, BuiltinWrapper, Objects};
use crate::builtins::BuiltinFunctions;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Objects>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: Environment::create_map_with_builtins(),
            outer: None,
        }
    }

    pub fn default() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    fn create_map_with_builtins() -> HashMap<String, Objects> {
        let mut map: HashMap<String, Objects> = HashMap::new();
        let map = HashMap::from_iter([(
            String::from("len"),
            Objects::Builtin(Rc::new(BuiltinWrapper::new(BuiltinFunction::Default(
                BuiltinFunctions::len,
            )))),
        )]);
        map
    }

    pub fn get(&self, key: String) -> Option<Objects> {
        let mut result = self.store.get(&key).cloned();
        if result.is_none() && self.outer.is_some() {
            result = self
                .outer
                .as_ref()
                .unwrap()
                .borrow()
                .store
                .get(&key)
                .cloned();
        }
        result
    }

    pub fn set(&mut self, key: String, value: Objects) {
        self.store.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn set_outer_env(&mut self, outer: Rc<RefCell<Environment>>) {
        self.outer = Some(outer);
    }

    pub fn new_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Environment {
        let mut new_env = Environment::default();
        new_env.set_outer_env(outer);
        new_env
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        panic!("Tried to equate to Environments");
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        panic!("Tried to compare Environments")
    }
}
