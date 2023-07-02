use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

use super::Expressions;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InfixExpression {
    operator: String,
    expression_left: Expressions,
    expression_right: Expressions,
}

impl InfixExpression {
    pub fn new(
        operator: String,
        expression_left: Expressions,
        expression_right: Expressions,
    ) -> Self {
        Self {
            operator,
            expression_left,
            expression_right,
        }
    }
    pub fn token(&self) -> &Token {
        match &self.expression_left {
            Expressions::Identifier(x) => x.token(),
            Expressions::BooleanLiteral(x) => x.token(),
            Expressions::IntegerLiteral(x) => x.token(),
            Expressions::IfExpression(x) => x.token(),
            Expressions::InfixExpression(x) => x.token(),
            Expressions::PrefixExpression(x) => x.token(),
            Expressions::CallExpression(x) => x.token(),
            Expressions::FunctionLiteral(x) => x.token(),
            Expressions::StringLiteral(x) => x.token(),
            Expressions::Empty => panic!("Tried to get token from empty expression"),
        }
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

    pub fn expression_right_mut(&mut self) -> &mut Expressions {
        &mut self.expression_right
    }

    pub fn expression_left_mut(&mut self) -> &mut Expressions {
        &mut self.expression_left
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        match &self.expression_left {
            Expressions::Identifier(x) => x.token_literal(),
            Expressions::BooleanLiteral(x) => x.token_literal(),
            Expressions::IntegerLiteral(x) => x.token_literal(),
            Expressions::IfExpression(x) => x.token_literal(),
            Expressions::InfixExpression(x) => x.token_literal(),
            Expressions::PrefixExpression(x) => x.token_literal(),
            Expressions::CallExpression(x) => x.token_literal(),
            Expressions::FunctionLiteral(x) => x.token_literal(),
            Expressions::StringLiteral(x) => x.token_literal(),
            Expressions::Empty => panic!("Tried to print token of empty expression"),
        }
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
