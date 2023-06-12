use crate::lexer::{Lexer, Token};
use crate::ast::{self, Identifier, Statement, LetStatement};
use crate::ast::Node;
use std::error::Error;
use std::mem::{discriminant, Discriminant};

pub struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser{
            lexer,
            curr_token: None,
            peek_token: None
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.take();
        let tok = self.lexer.next_token();
        self.peek_token = Some(tok);
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Box<dyn Error>> {
        let mut program = ast::Program::new();
        while let Some(token) = &self.curr_token {
            match token {
                Token::EOF => break,
                _ => {
                    let statement = self.parse_statement();
                    if let Some(statement) = statement {
                        program.statements.push(statement);
                    }
                    self.next_token();
                }
            }
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match &self.curr_token {
            Some(Token::LET) => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let let_token = self.curr_token.take();
        if let_token.is_none(){
            return None;
        }
        let let_token = let_token.unwrap();

        let exp_token = discriminant(&Token::IDENT("".into()));
        if !&self.expect_peek(exp_token){
            return None;
        }
        let ident_token = self.curr_token.take();
        if ident_token.is_none(){
            return None;
        }
        let ident_token = ident_token.unwrap();
        let ident_literal = match &ident_token {
            Token::IDENT(value) => value.clone(),
            _ => unreachable!()
        };
        let ident = Identifier::new(ident_token, ident_literal);

        let exp_token = discriminant(&Token::ASSIGN);
        if !&self.expect_peek(exp_token){
            return None;
        }
        let break_token = discriminant(&Token::SEMICOLON);
        while let Some(tok) = &self.curr_token {
            if discriminant::<Token>(&tok) == break_token {
                break;
            } else {
                self.next_token();
            }
        }
        Some(Box::new(LetStatement::new(
            let_token,
            ident,
        )))
    }

    fn expect_peek(&mut self, token_type: Discriminant<Token>) -> bool {
        match &self.peek_token {
            Some(tok) if discriminant::<Token>(&tok) == token_type => {
                self.next_token();
                true
            },
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let mut l = Lexer::new(input.into());
        let mut parser = Parser::new(l);

        let program = parser.parse_program();
        assert!(program.is_ok());
        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        let expexted_identifiers = vec![
            "x",
            "y",
            "foobar"
        ];

        for (i, ident) in expexted_identifiers.iter().enumerate() {
            let stmt = &program.statements[i];
            assert_eq!(stmt.token_literal(), "LET");
            assert_eq!(stmt.name().value(), *ident);
        }
    }
}
