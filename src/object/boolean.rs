use std::fmt::Display;

use super::{Object, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &bool {
        &self.value
    }
}

impl Object for Boolean {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Boolean
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
