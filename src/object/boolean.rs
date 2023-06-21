use super::{Object, ObjectType};

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
        "BOOLEAN".into()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
