use super::Object;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Integer {
    value: isize,
}

impl Integer {
    pub fn new(value: isize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &isize {
        &self.value
    }
}

impl Object for Integer {
    fn obj_type(&self) -> super::ObjectType {
        "INTEGER".into()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
