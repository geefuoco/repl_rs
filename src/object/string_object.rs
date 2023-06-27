use std::fmt::Display;

use super::{Object, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct StringObject {
    pub value: String,
}

impl StringObject {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Object for StringObject {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::String
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Display for StringObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
