use crate::ast::{Expression, Node, Statement, Token};
use std::fmt::Display;

use super::AsAny;

pub struct ExpressionStatement {
    token: Token,
    expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn expression(&self) -> &Box<dyn Expression> {
        &self.expression
    }
}

impl AsAny for ExpressionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
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
