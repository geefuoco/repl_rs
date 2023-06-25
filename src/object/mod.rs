use std::{
    error::Error,
    fmt::{Debug, Display},
};

mod boolean;
mod environment;
mod error;
mod integer;
mod null;
mod return_object;
pub use boolean::Boolean;
pub use environment::Environment;
pub use error::Error as ErrorObject;
pub use integer::Integer;
pub use null::Null;
pub use return_object::Return;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Objects {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Return),
    Error(ErrorObject),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectTypes {
    Integer,
    Boolean,
    Null,
    Return,
    Error,
}

impl Display for Objects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objects::Integer(_) => write!(f, "INTEGER"),
            Objects::Boolean(_) => write!(f, "BOOLEAN"),
            Objects::Null(_) => write!(f, "NULL"),
            Objects::Return(_) => write!(f, "RETURN"),
            Objects::Error(_) => write!(f, "ERROR"),
        }
    }
}

impl Objects {
    pub fn as_integer(self) -> Option<Integer> {
        match self {
            Objects::Integer(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_boolean(self) -> Option<Boolean> {
        match self {
            Objects::Boolean(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_null(self) -> Option<Null> {
        match self {
            Objects::Null(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_return(self) -> Option<Return> {
        match self {
            Objects::Return(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_err(self) -> Option<ErrorObject> {
        match self {
            Objects::Error(x) => Some(x),
            _ => None,
        }
    }
}

impl Object for Objects {
    fn obj_type(&self) -> ObjectTypes {
        match self {
            x => x.obj_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            x => x.inspect(),
        }
    }

    fn is_err(&self) -> bool {
        match self {
            Objects::Error(x) => true,
            _ => false,
        }
    }
}

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

pub trait Object: Debug {
    fn obj_type(&self) -> ObjectTypes;
    fn inspect(&self) -> String;
    fn is_err(&self) -> bool {
        match self.obj_type() {
            ObjectTypes::Error => true,
            _ => false,
        }
    }
}
