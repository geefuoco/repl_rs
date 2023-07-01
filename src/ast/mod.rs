use crate::lexer::Token;
use std::fmt::{Debug, Display};

pub mod block_statement;
pub mod boolean_literal;
pub mod call_expression;
pub mod expression_statement;
pub mod function_literal;
pub mod identifier;
pub mod if_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod return_statement;
pub mod string_literal;

pub use block_statement::BlockStatement;
pub use boolean_literal::BooleanLiteral;
pub use call_expression::CallExpression;
pub use expression_statement::ExpressionStatement;
pub use function_literal::FunctionLiteral;
pub use identifier::Identifier;
pub use if_expression::IfExpression;
pub use infix_expression::InfixExpression;
pub use integer_literal::IntegerLiteral;
pub use let_statement::LetStatement;
pub use prefix_expression::PrefixExpression;
pub use return_statement::ReturnStatement;
pub use string_literal::StringLiteral;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Expressions {
    Identifier(Identifier),
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    IfExpression(IfExpression),
    InfixExpression(Box<InfixExpression>),
    PrefixExpression(Box<PrefixExpression>),
    CallExpression(Box<CallExpression>),
    FunctionLiteral(FunctionLiteral),
    StringLiteral(StringLiteral),
    #[default]
    Empty,
}

impl Expressions {
    pub fn as_call_expression(self) -> Option<Box<CallExpression>> {
        match self {
            Expressions::CallExpression(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_prefix_expression(self) -> Option<Box<PrefixExpression>> {
        match self {
            Expressions::PrefixExpression(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_function_literal(self) -> Option<FunctionLiteral> {
        match self {
            Expressions::FunctionLiteral(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_infix_expression(self) -> Option<Box<InfixExpression>> {
        match self {
            Expressions::InfixExpression(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_if_expression(self) -> Option<IfExpression> {
        match self {
            Expressions::IfExpression(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_integer_literal(self) -> Option<IntegerLiteral> {
        match self {
            Expressions::IntegerLiteral(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_boolean_literal(self) -> Option<BooleanLiteral> {
        match self {
            Expressions::BooleanLiteral(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_identifier(self) -> Option<Identifier> {
        match self {
            Expressions::Identifier(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_string_literal(self) -> Option<StringLiteral> {
        match self {
            Expressions::StringLiteral(x) => Some(x),
            _ => None,
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressions::Identifier(x) => write!(f, "{}", x),
            Expressions::BooleanLiteral(x) => write!(f, "{}", x),
            Expressions::IntegerLiteral(x) => write!(f, "{}", x),
            Expressions::IfExpression(x) => write!(f, "{}", x),
            Expressions::InfixExpression(x) => write!(f, "{}", x),
            Expressions::PrefixExpression(x) => write!(f, "{}", x),
            Expressions::CallExpression(x) => write!(f, "{}", x),
            Expressions::FunctionLiteral(x) => write!(f, "{}", x),
            Expressions::StringLiteral(x) => write!(f, "{}", x),
            Expressions::Empty => panic!("Cannot display an empty expression"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Statements {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
    #[default]
    Empty,
}

impl Statements {
    pub fn as_let_statement(self) -> Option<LetStatement> {
        match self {
            Statements::LetStatement(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_return_statement(self) -> Option<ReturnStatement> {
        match self {
            Statements::ReturnStatement(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_expression_statement(self) -> Option<ExpressionStatement> {
        match self {
            Statements::ExpressionStatement(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_block_statement(self) -> Option<BlockStatement> {
        match self {
            Statements::BlockStatement(v) => Some(v),
            _ => None,
        }
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statements::LetStatement(x) => write!(f, "{}", x),
            Statements::ReturnStatement(x) => write!(f, "{}", x),
            Statements::ExpressionStatement(x) => write!(f, "{}", x),
            Statements::BlockStatement(x) => write!(f, "{}", x),
            Statements::Empty => panic!("Cannot display an empty statement"),
        }
    }
}

pub trait Node: Display {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Statements>,
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
            match &self.statements[0] {
                Statements::LetStatement(stmt) => stmt.token_literal(),
                Statements::ReturnStatement(stmt) => stmt.token_literal(),
                Statements::ExpressionStatement(stmt) => stmt.token_literal(),
                Statements::BlockStatement(stmt) => stmt.token_literal(),
                Statements::Empty => panic!("Program encountered empty statement"),
            }
        } else {
            ""
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
        let mut v: Vec<Statements> = Vec::new();
        let name = Identifier::new(Token::Ident("my_var".into()));
        let value = Identifier::new(Token::Ident("another_var".into()));
        let let_statement = LetStatement::new(Token::Let, name, Expressions::Identifier(value));
        v.push(Statements::LetStatement(let_statement));

        let mut test_str = String::new();
        let program = Program { statements: v };
        write!(test_str, "{}", program).unwrap();
        assert_eq!("let my_var = another_var;\n", test_str);
    }
}
