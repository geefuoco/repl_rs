use crate::ast::AsAny;

use super::{Object, ObjectType, ObjectTypes};

#[derive(Debug)]
pub struct Null {}

impl Object for Null {
    fn obj_type(&self) -> ObjectType {
        ObjectTypes::NULL
    }

    fn inspect(&self) -> String {
        String::from("null")
    }
}

impl AsAny for Null {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
