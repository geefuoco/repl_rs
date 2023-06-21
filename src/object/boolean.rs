use crate::ast::AsAny;

use super::{Object, ObjectType, ObjectTypes};

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
    fn obj_type(&self) -> ObjectType {
        ObjectTypes::BOOLEAN
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
impl AsAny for Boolean {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
