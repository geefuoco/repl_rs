use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IntegerLiteral {
    token: Token,
    value: isize
}

impl IntegerLiteral {
    pub fn new(token: Token) -> Self {
        match token {
            Token::Integer(_) => {
                let value = token.literal().parse::<isize>().expect("value was not an isize");
                Self { token, value}
            },
            _ => panic!("Tried to make an integer literal with {}", token),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> isize {
        self.value
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())?;
        Ok(())
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
