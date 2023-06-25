use std::fmt::Display;

use super::{Object, ObjectTypes, Objects};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Return {
    value: Box<Objects>,
}

impl Return {
    pub fn new(value: Objects) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    pub fn value(&self) -> &Box<Objects> {
        &self.value
    }
}

impl Object for Return {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Return
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.inspect())
    }
}
impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
