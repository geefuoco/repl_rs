use crate::ast::{identifier::Identifier, Expression, Node, Statement, Token};
use std::fmt::Display;

use super::AsAny;

pub struct ReturnStatement {
    token: Token,
    return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<dyn Expression>) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn return_value(&self) -> &Box<dyn Expression> {
        &self.return_value
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)?;
        Ok(())
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl AsAny for ReturnStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}
