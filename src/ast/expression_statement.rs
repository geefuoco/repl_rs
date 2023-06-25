use crate::ast::{Expressions, Node, Statement, Token};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    token: Token,
    expression: Expressions,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Expressions) -> Self {
        Self { token, expression }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn expression(&self) -> &Expressions {
        &self.expression
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)?;
        Ok(())
    }
}
