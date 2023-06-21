use super::{Object, ObjectType, Objects};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Return {
    value: Box<dyn Object>,
}

impl Return {
    pub fn new(value: Objects) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Objects {
        &self.value
    }
}

impl Object for Return {
    fn obj_type(&self) -> ObjectType {
        "RETURN".into()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.inspect())
    }
}
