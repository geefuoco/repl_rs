use crate::ast::AsAny;
use std::fmt::{Display, Debug};

mod boolean;
mod integer;
mod null;
pub use boolean::Boolean;
pub use integer::Integer;
pub use null::Null;

type ObjectType = ObjectTypes;

pub trait Object: AsAny + Debug{
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub enum ObjectTypes {
    INTEGER,
    BOOLEAN,
    NULL,
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
