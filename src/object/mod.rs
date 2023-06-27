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
mod function;
mod string_object;
pub use boolean::Boolean;
pub use environment::Environment;
pub use error::Error as ErrorObject;
pub use function::Function;
pub use integer::Integer;
pub use null::Null;
pub use return_object::Return;
pub use string_object::StringObject;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Objects {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Return),
    Error(ErrorObject),
    Function(Function),
    String(StringObject)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ObjectTypes {
    Integer,
    Boolean,
    Null,
    Return,
    Error,
    Function,
    String
}

impl Display for ObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypes::Integer => write!(f, "INTEGER"),
            ObjectTypes::Boolean => write!(f, "BOOLEAN"),
            ObjectTypes::Null => write!(f, "NULL"),
            ObjectTypes::Return => write!(f, "RETURN"),
            ObjectTypes::Error => write!(f, "ERROR"),
            ObjectTypes::Function => write!(f, "FUNCTION"),
            ObjectTypes::String => write!(f, "STRING")
        }
    }
}

impl Display for Objects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objects::Integer(x) => write!(f, "{}", x.obj_type()),
            Objects::Boolean(x) => write!(f, "{}", x.obj_type()),
            Objects::Null(x) => write!(f, "{}", x.obj_type()),
            Objects::Return(x) => write!(f, "{}", x.obj_type()),
            Objects::Error(x) => write!(f, "{}", x.obj_type()),
            Objects::Function(x) => write!(f, "{}", x.obj_type()),
            Objects::String(x) => write!(f, "{}", x.obj_type())
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
    pub fn as_fn(self) -> Option<Function> {
        match self {
            Objects::Function(x) => Some(x),
            _ => None
        }
    }
    pub fn as_err(self) -> Option<ErrorObject> {
        match self {
            Objects::Error(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_str(self) -> Option<StringObject> {
        match self {
            Objects::String(x) => Some(x),
            _ => None
        }
    }
}

impl Object for Objects {
    fn obj_type(&self) -> ObjectTypes {
        match self {
            Objects::Integer(x) => x.obj_type(),
            Objects::Boolean(x) => x.obj_type(),
            Objects::Null(x) => x.obj_type(),
            Objects::Return(x) => x.obj_type(),
            Objects::Error(x) => x.obj_type(),
            Objects::Function(x) => x.obj_type(),
            Objects::String(x) => x.obj_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Objects::Integer(x) => x.inspect(),
            Objects::Boolean(x) => x.inspect(),
            Objects::Null(x) => x.inspect(),
            Objects::Return(x) => x.inspect(),
            Objects::Error(x) => x.inspect(),
            Objects::Function(x) => x.inspect(),
            Objects::String(x) => x.inspect()
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
