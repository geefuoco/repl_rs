use crate::ast::{Expression, Node, Statement, Token};
use std::fmt::Display;

use super::AsAny;

pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> Self {
        Self { token, statements }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }
}

impl AsAny for BlockStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}
