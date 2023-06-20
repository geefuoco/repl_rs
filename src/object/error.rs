use crate::ast::AsAny;

use super::{Object, ObjectType, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message: String) -> Self{
        Self{message}
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Object for Error {
    fn obj_type(&self) -> ObjectType {
        ObjectTypes::ERROR
    }

    fn inspect(&self) -> String {
        format!("{}", self.message)
    }
}
impl AsAny for Error {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
