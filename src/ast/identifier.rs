use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::AsAny;
#[derive(Clone)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())?;
        Ok(())
    }
}

impl AsAny for Identifier {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
