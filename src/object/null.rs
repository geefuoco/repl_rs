use std::fmt::Display;

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
impl Display for Null{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
