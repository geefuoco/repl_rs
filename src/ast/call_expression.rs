use super::Expressions;
use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CallExpression {
    token: Token,
    function: Expressions,
    arguments: Vec<Expressions>,
}

impl CallExpression {
    pub fn new(token: Token, function: Expressions, arguments: Vec<Expressions>) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn function(&self) -> &Expressions {
        &self.function
    }

    pub fn arguments(&self) -> &[Expressions] {
        &self.arguments
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.function.token_literal())?;
        let n = self.arguments.len();
        for (i, param) in self.arguments().iter().enumerate() {
            write!(f, "{}", param)?;
            if i < n - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}
