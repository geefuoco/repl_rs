use crate::ast::AsAny;

use super::{Object, ObjectType, ObjectTypes};

#[derive(Debug)]
pub struct Return {
    pub value: Box<dyn Object>,
}

impl Return {
    pub fn new(value: Box<dyn Object>) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Box<dyn Object> {
        &self.value
    }
}

impl Object for Return {
    fn obj_type(&self) -> ObjectType {
        ObjectTypes::RETURN
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.inspect())
    }
}
impl AsAny for Return {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
