use super::{Object, ObjectType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Null {}

impl Object for Null {
    fn obj_type(&self) -> ObjectType {
        "NULL".into()
    }

    fn inspect(&self) -> String {
        String::from("null")
    }
}
