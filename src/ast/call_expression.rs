use super::{AsAny, Identifier};
use crate::ast::{BlockStatement, Expression, Node, Token};
use std::fmt::Display;

pub struct CallExpression {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(token: Token, function: Box<dyn Expression>, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self{
            token,
            function,
            arguments
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn function(&self) -> &dyn Expression {
        self.function.as_ref()
    }

    pub fn arguments(&self) -> &Vec<Box<dyn Expression>> {
        self.arguments.as_ref()
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl AsAny for CallExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.function.token_literal())?;
        let n = self.arguments.len();
        for (i, param) in self.arguments().iter().enumerate(){
            write!(f, "{}", param)?;
            if i < n-1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
