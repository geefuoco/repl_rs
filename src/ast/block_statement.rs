use crate::ast::{Node, Statement, Token};
use std::fmt::Display;

use super::Statements;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockStatement {
    token: Token,
    statements: Vec<Statements>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Statements>) -> Self {
        Self { token, statements }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn statements(&self) -> &[Statements] {
        &self.statements
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
