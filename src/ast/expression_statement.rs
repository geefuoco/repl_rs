use crate::ast::{Expressions, Node, Statement, Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExpressionStatement {
    expression: Expressions,
}

impl ExpressionStatement {
    pub fn new(expression: Expressions) -> Self {
        Self { expression }
    }
    pub fn token(&self) -> &Token {
        match &self.expression {
            Expressions::Identifier(x) => x.token(),
            Expressions::BooleanLiteral(x) => x.token(),
            Expressions::IntegerLiteral(x) => x.token(),
            Expressions::IfExpression(x) => x.token(),
            Expressions::InfixExpression(x) => x.token(),
            Expressions::PrefixExpression(x) => x.token(),
            Expressions::CallExpression(x) => x.token(),
            Expressions::FunctionLiteral(x) => x.token(),
            Expressions::StringLiteral(x) => x.token(),
            Expressions::Empty => panic!("Token was empty"),
        }
    }

    pub fn expression(&self) -> &Expressions {
        &self.expression
    }

    pub fn expression_mut(&mut self) -> &mut Expressions {
        &mut self.expression
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        self.token().literal()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)?;
        Ok(())
    }
}
