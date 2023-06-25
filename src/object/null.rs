use super::{Object, ObjectTypes};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Null {}

impl Object for Null {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Null
    }

    fn inspect(&self) -> String {
        String::from("null")
    }
}
