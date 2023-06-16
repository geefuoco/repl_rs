use super::{AsAny, Identifier};
use crate::ast::{BlockStatement, Expression, Node, Token};
use std::fmt::Display;

pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Identifier>,
    body: BlockStatement
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Self {
            token,
            parameters,
            body
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn body(&self) -> &BlockStatement {
        &self.body
    }

    pub fn parameters(&self) -> &[Identifier] {
        self.parameters.as_ref()
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl AsAny for FunctionLiteral {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (", self.token_literal())?;
        for param in &self.parameters {
            write!(f, "{}, ", param)?;
        }
        write!(f, ") {{ {} ", self.body)?;
        write!(f, "}}")
    }
}
