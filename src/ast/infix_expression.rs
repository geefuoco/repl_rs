use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::AsAny;

pub struct InfixExpression {
    token: Token,
    operator: String,
    expression_left: Box<dyn Expression>,
    expression_right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        operator: String,
        expression_left: Box<dyn Expression>,
        expression_right: Box<dyn Expression>,
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

    pub fn expression_right(&self) -> &Box<dyn Expression> {
        &self.expression_right
    }

    pub fn expression_left(&self) -> &Box<dyn Expression> {
        &self.expression_left
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl AsAny for InfixExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
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
            "({}{}{})",
            self.expression_left, self.operator, self.expression_right
        )?;
        Ok(())
    }
}
