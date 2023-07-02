use super::{Expressions, OptionalBlockStatement};
use crate::ast::{block_statement::BlockStatement, Expression, Node, Token};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfExpression {
    token: Token,
    condition: Box<Expressions>,
    consequence: BlockStatement,
    //The wrapper for option is here to allow for impl display block
    alternative: OptionalBlockStatement<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Expressions,
        consequence: BlockStatement,
        alternative: OptionalBlockStatement<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn consequence(&self) -> &BlockStatement {
        &self.consequence
    }

    pub fn consequence_mut(&mut self) -> &mut BlockStatement {
        &mut self.consequence
    }

    pub fn condition(&self) -> &Box<Expressions> {
        &self.condition
    }
    pub fn condition_mut(&mut self) -> &mut Expressions {
        self.condition.as_mut()
    }

    pub fn alternative(&self) -> Option<&BlockStatement> {
        self.alternative.0.as_ref()
    }

    pub fn alternative_mut(&mut self) -> Option<&mut BlockStatement> {
        self.alternative.0.as_mut()
    }
}

impl<T: Display> Display for OptionalBlockStatement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_some() {
            write!(f, "{}", &self.0.as_ref().unwrap())?;
        }
        Ok(())
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {{ {} }}", self.condition, self.consequence)?;
        if self.alternative.0.is_some() {
            write!(f, " else {{ {} }}", self.alternative)?;
        }
        Ok(())
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}
