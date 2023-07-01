use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::Expressions;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PrefixExpression {
    token: Token,
    operator: String,
    expression_right: Expressions,
}

impl PrefixExpression {
    pub fn new(token: Token, expression_right: Expressions) -> Self {
        let operator = String::from(token.literal());
        Self {
            token,
            operator,
            expression_right,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn expression_right(&self) -> &Expressions {
        &self.expression_right
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.expression_right)?;
        Ok(())
    }
}
