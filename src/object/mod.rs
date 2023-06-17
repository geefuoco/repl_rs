use crate::ast::AsAny;
use std::{fmt::{Debug, Display}, error::Error, str::FromStr};

mod boolean;
mod integer;
mod null;
pub use boolean::Boolean;
pub use integer::Integer;
pub use null::Null;

type ObjectType = ObjectTypes;


pub struct CastError {
    obj_type: String
}
impl Error for CastError {}

impl Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred when trying to cast to {} from dyn Object trait", self.obj_type)
    }
}
impl Debug for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred when trying to cast to {} from dyn Object trait", self.obj_type)
    }
}

pub trait Object: AsAny + Debug {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_integer(&self) -> Result<Integer, CastError> {
        match self.inspect().parse::<isize>() {
            Ok(v) => Ok(Integer::new(v)),
            _ => Err(CastError{obj_type: String::from("Integer")})
        }
    }
    fn as_boolean(&self) -> Result<Boolean, CastError> {
        match self.inspect().parse::<bool>() {
            Ok(v) => Ok(Boolean::new(v)),
            _ => Err(CastError{obj_type: String::from("Boolean")})
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
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
