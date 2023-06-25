use super::{Object, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Object for Error {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Error
    }

    fn inspect(&self) -> String {
        format!("{}", self.message)
    }
}
