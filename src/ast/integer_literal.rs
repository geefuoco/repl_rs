use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::AsAny;
pub struct IntegerLiteral {
    token: Token,
    value: isize,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: isize) -> Self {
        Self { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &isize {
        &self.value
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())?;
        Ok(())
    }
}

impl AsAny for IntegerLiteral {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}