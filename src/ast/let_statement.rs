use crate::ast::{identifier::Identifier, Node, Statement, Token};
use std::fmt::Display;

use super::Expressions;

#[derive(Debug, Clone)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expressions,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Expressions) -> Self {
        LetStatement { token, name, value }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn value(&self) -> &Expressions {
        &self.value
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
        )?;
        Ok(())
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
