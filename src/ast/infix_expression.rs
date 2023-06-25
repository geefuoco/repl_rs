use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::Expressions;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    token: Token,
    operator: String,
    expression_left: Expressions,
    expression_right: Expressions,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        operator: String,
        expression_left: Expressions,
        expression_right: Expressions,
    ) -> Self {
        Self {
            token,
            operator,
            expression_left,
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

    pub fn expression_left(&self) -> &Expressions {
        &self.expression_left
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.expression_left, self.operator, self.expression_right
        )?;
        Ok(())
    }
}
