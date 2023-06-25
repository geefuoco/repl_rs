use crate::ast::{Node, Statement, Token};
use std::fmt::Display;

use super::Expressions;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    token: Token,
    return_value: Expressions,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Expressions) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn return_value(&self) -> &Expressions {
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

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}
