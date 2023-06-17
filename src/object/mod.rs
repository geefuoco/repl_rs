use crate::ast::AsAny;
use std::fmt::Display;

mod integer;
mod boolean;
mod null;
pub use integer::Integer;
pub use boolean::Boolean;
pub use null::Null;

type ObjectType = ObjectTypes;

pub trait Object: AsAny {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

enum ObjectTypes {
    INTEGER,
    BOOLEAN,
    NULL
}

impl Display for ObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypes::INTEGER => write!(f, "INTEGER"),
            ObjectTypes::BOOLEAN => write!(f, "BOOLEAN"),
            ObjectTypes::NULL => write!(f, "NULL"),
        }
    }
}

