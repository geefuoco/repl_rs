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
