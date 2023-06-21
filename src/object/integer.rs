use crate::ast::AsAny;

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
    fn obj_type(&self) -> super::ObjectType {
        ObjectTypes::INTEGER
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
impl AsAny for Integer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
