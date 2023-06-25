use std::collections::HashMap;

use super::Objects;

pub struct Environment {
    store: HashMap<String, Objects>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: String) -> Option<Objects> {
        self.store.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: Objects) {
        self.store.insert(key, value);
    }
}
