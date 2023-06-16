use crate::lexer::Token;
use std::any::Any;
use std::fmt::{Debug, Display};

pub mod block_statement;
pub mod boolean_literal;
pub mod expression_statement;
pub mod function_literal;
pub mod identifier;
pub mod if_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod return_statement;

pub use block_statement::BlockStatement;
pub use boolean_literal::BooleanLiteral;
pub use expression_statement::ExpressionStatement;
pub use function_literal::FunctionLiteral;
pub use identifier::Identifier;
pub use if_expression::IfExpression;
pub use infix_expression::InfixExpression;
pub use integer_literal::IntegerLiteral;
pub use let_statement::LetStatement;
pub use prefix_expression::PrefixExpression;
pub use return_statement::ReturnStatement;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

pub trait Node: Display {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node + AsAny {
    fn statement_node(&self);
}

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement {{ {} }}", self.token_literal())
    }
}

pub trait Expression: Node + AsAny {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        let v = Vec::new();
        Program { statements: v }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?
        }
        Ok(())
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        ""
    }
}

pub struct OptionalBlockStatement<T>(Option<T>);

impl<T> OptionalBlockStatement<T> {
    pub fn new(option: Option<T>) -> Self {
        Self(option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn test_string() {
        let mut v: Vec<Box<dyn Statement>> = Vec::new();
        v.push(Box::new(let_statement::LetStatement {
            token: Token::Let,
            name: identifier::Identifier::new(Token::Ident("my_var".into()), "my_var".into()),
            value: Box::new(identifier::Identifier::new(
                Token::Ident("another_var".into()),
                "another_var".into(),
            )),
        }));

        let mut test_str = String::new();
        let program = Program { statements: v };
        write!(test_str, "{}", program).unwrap();
        assert_eq!("let my_var = another_var;\n", test_str);
    }
}
