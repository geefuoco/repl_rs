use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BooleanLiteral {
    token: Token,
    value: bool,
}

impl BooleanLiteral {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &bool {
        &self.value
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())?;
        Ok(())
    }
}

impl Expression for BooleanLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
