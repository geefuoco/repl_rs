use std::collections::HashMap;

use super::Object;

pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: String) -> Option<Box<dyn Object>> {
        self.store.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: Box<dyn Object>) {
        self.store.insert(key, value);
    }
}
