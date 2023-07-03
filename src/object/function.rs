use crate::ast::{BlockStatement, Identifier};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::{Environment, Object, ObjectTypes};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Function {
    parameters: Vec<Identifier>,
    body: BlockStatement,
    env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    ) -> Self {
        Function {
            parameters,
            body,
            env,
        }
    }

    pub fn parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut BlockStatement {
        &mut self.body
    }

    pub fn environment(&self) -> &Rc<RefCell<Environment>> {
        &self.env
    }
}

impl Object for Function {
    fn obj_type(&self) -> ObjectTypes {
        ObjectTypes::Function
    }

    fn inspect(&self) -> String {
        let mut s = String::new();
        s.push_str("fn(");
        let params: Vec<String> = self.parameters.iter().map(|i| i.to_string()).collect();
        let len = params.len();
        for (i, p) in params.iter().enumerate() {
            s.push_str(p.as_str());
            if i < len - 1 {
                s.push_str(",")
            }
        }
        s.push_str(")\n");
        s.push_str(self.body.to_string().as_str());
        s.push_str("\n}");
        s
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.obj_type())
    }
}
