use crate::ast::{Expression, Node, Token};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Identifier {
    token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        match token  {
            Token::Ident(_) =>Self { token },
            _ => panic!("Tried to make an identifier with a {} token", token)
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn value(&self) -> &str {
        self.token_literal()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())?;
        Ok(())
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
