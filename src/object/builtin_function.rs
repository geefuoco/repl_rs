use super::{Object, ObjectTypes, Objects};

type BuiltinFunction = fn(args: &[Objects]) -> Objects;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BuiltinWrapper {
    func: BuiltinFunction,
}

impl BuiltinWrapper {
    pub const fn new(func: BuiltinFunction) -> Self {
        Self { func }
    }
    pub fn func(&self) -> &BuiltinFunction {
        &self.func
    }
}

impl Object for BuiltinWrapper {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Builtin
    }

    fn inspect(&self) -> String {
        String::from("Builtin Function")
    }
}
