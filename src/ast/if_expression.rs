use super::{AsAny, OptionalBlockStatement, Statement};
use crate::ast::{block_statement::BlockStatement, Expression, Node, Token};
use std::fmt::Display;

pub struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: OptionalBlockStatement<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: OptionalBlockStatement<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
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

    pub fn condition(&self) -> &dyn Expression {
        self.condition.as_ref()
    }

    pub fn alternative(&self) -> &Option<BlockStatement> {
        &self.alternative.0
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

impl AsAny for IfExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
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
