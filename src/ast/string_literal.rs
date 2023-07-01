use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StringLiteral {
    token: Token,
    value: String,
}

impl StringLiteral {
    pub fn new(token: Token) -> Self {
        let value = String::from(token.literal());
        Self { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())?;
        Ok(())
    }
}

impl Expression for StringLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for StringLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
