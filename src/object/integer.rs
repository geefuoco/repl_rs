use std::fmt::Display;

use super::{Object, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Integer {
    value: isize,
}

impl Integer {
    pub fn new(value: isize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &isize {
        &self.value
    }
}

impl Object for Integer {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Integer
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
impl Display for Integer{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
