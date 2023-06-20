use crate::ast::AsAny;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

mod boolean;
mod integer;
mod null;
mod return_object;
pub use boolean::Boolean;
pub use integer::Integer;
pub use null::Null;
pub use return_object::Return;

type ObjectType = ObjectTypes;

pub struct CastError {
    obj_type: String,
}
impl Error for CastError {}

impl Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error occurred when trying to cast to {} from dyn Object trait",
            self.obj_type
        )
    }
}
impl Debug for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error occurred when trying to cast to {} from dyn Object trait",
            self.obj_type
        )
    }
}

pub trait Object: AsAny + Debug {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_integer(&self) -> Result<Integer, CastError> {
        match self.inspect().parse::<isize>() {
            Ok(v) => Ok(Integer::new(v)),
            _ => Err(CastError {
                obj_type: String::from("Integer"),
            }),
        }
    }
    fn as_boolean(&self) -> Result<Boolean, CastError> {
        match self.inspect().parse::<bool>() {
            Ok(v) => Ok(Boolean::new(v)),
            _ => Err(CastError {
                obj_type: String::from("Boolean"),
            }),
        }
    }
    fn as_null(&self) -> Result<Null, CastError> {
        match self.inspect().as_ref() {
            "null" => Ok(Null {}),
            _ => Err(CastError {
                obj_type: String::from("Null"),
            }),
        }
    }
    fn as_return_value(&self) -> Result<Return, CastError> {
        match self.obj_type() {
            ObjectTypes::INTEGER => Ok(Return::new(Box::new(Integer::new(
                self.inspect().parse::<isize>().unwrap(),
            )))),
            ObjectTypes::BOOLEAN => Ok(Return::new(Box::new(Boolean::new(
                self.inspect().parse::<bool>().unwrap(),
            )))),
            ObjectTypes::NULL => Ok(Return::new(Box::new(Null {}))),
            ObjectTypes::RETURN => Err(CastError {
                obj_type: String::from("Return"),
            }),
        }
    }

    fn is_return_value(&self) -> bool {
        match self.obj_type() {
            ObjectTypes::RETURN => true,
            _ => false,
        }
    }
    fn is_integer(&self) -> bool {
        match self.obj_type() {
            ObjectTypes::INTEGER => true,
            _ => false,
        }
    }
    fn is_boolean(&self) -> bool {
        match self.obj_type() {
            ObjectTypes::BOOLEAN => true,
            _ => false,
        }
    }
    fn is_null(&self) -> bool {
        match self.obj_type() {
            ObjectTypes::NULL => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectTypes {
    INTEGER,
    BOOLEAN,
    NULL,
    RETURN,
}

impl Display for ObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypes::INTEGER => write!(f, "INTEGER"),
            ObjectTypes::BOOLEAN => write!(f, "BOOLEAN"),
            ObjectTypes::NULL => write!(f, "NULL"),
            ObjectTypes::RETURN => write!(f, "RETURN"),
        }
    }
}
