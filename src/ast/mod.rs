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

#[derive(Default, Debug, Clone)]
pub enum Expressions {
    Identifier(Identifier),
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    IfExpression(IfExpression),
    InfixExpression(Box<InfixExpression>),
    PrefixExpression(Box<PrefixExpression>),
    CallExpression(Box<CallExpression>),
    FunctionLiteral(FunctionLiteral),
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
}

impl Expression for Expressions {
    fn expression_node(&self) {
        match self {
            Expressions::Empty => panic!("Tried to get expression node from empty expression"),
            Expressions::Identifier(_) => todo!(),
            Expressions::BooleanLiteral(_) => todo!(),
            Expressions::IntegerLiteral(_) => todo!(),
            Expressions::IfExpression(_) => todo!(),
            Expressions::InfixExpression(_) => todo!(),
            Expressions::PrefixExpression(_) => todo!(),
            Expressions::CallExpression(_) => todo!(),
            Expressions::FunctionLiteral(_) => todo!(),
        }
    }
}

impl Node for Expressions {
    fn token_literal(&self) -> &str {
        match self {
            x => x.token_literal(),
        }
    }
}

impl Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            x => write!(f, "{}", self),
        }
    }
}

#[derive(Default, Debug, Clone)]
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

// impl Statement for Statements {
//     fn statement_node(&self) {
//         match self {
//             Statements::LetStatement(x) => x.statement_node(),
//             Statements::ReturnStatement(x) => x.statement_node(),
//             Statements::ExpressionStatement(x) => x.statement_node(),
//             Statements::BlockStatement(x) => x.statement_node(),
//             Statements::Empty => panic!("Tried to get statement node from empy statement"),
//         }
//     }
// }
//
impl Node for Statements {
    fn token_literal(&self) -> &str {
        match self {
            Statements::LetStatement(x) =>x.token_literal(),
            Statements::ReturnStatement(x) =>x.token_literal(),
            Statements::ExpressionStatement(x) =>x.token_literal() ,
            Statements::BlockStatement(x) => x.token_literal(),
            Statements::Empty => panic!("Tried to get token from empty statement"),
        }
    }
}

impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            x => write!(f, "{}", self),
        }
    }
}

pub trait Node: Display {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement {{ {} }}", self.token_literal())
    }
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
            return self.statements[0].token_literal();
        }
        ""
    }
}

#[derive(Debug, Clone)]
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
        let name = Identifier::new(Token::Ident("my_var".into()), "my_var".into());
        let value = Identifier::new(Token::Ident("another_var".into()), "another_var".into());
        let let_statement = LetStatement::new(Token::Let, name, Expressions::Identifier(value));
        v.push(Statements::LetStatement(let_statement));

        let mut test_str = String::new();
        let program = Program { statements: v };
        write!(test_str, "{}", program).unwrap();
        assert_eq!("let my_var = another_var;\n", test_str);
    }
}
