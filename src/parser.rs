use crate::ast::expression_statement::ExpressionStatement;
use crate::ast::infix_expression::InfixExpression;
use crate::ast::integer_literal::IntegerLiteral;
use crate::ast::prefix_expression::PrefixExpression;
use crate::ast::return_statement::ReturnStatement;
use crate::{
    ast::{identifier::Identifier, AsAny, Expression, Program, Statement},
    lexer::{Lexer, Token},
};
use std::collections::HashMap;
use std::error::Error;
use std::mem::discriminant;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Priority {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

type PrefixParseFn = fn(p: &mut Parser) -> Box<dyn Expression>;
type InfixParseFn = fn(p: &mut Parser, expresion: Box<dyn Expression>) -> Box<dyn Expression>;

pub struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<String, PrefixParseFn>,
    infix_parse_fns: HashMap<String, InfixParseFn>,
    precedences: HashMap<String, Priority>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let precedences = HashMap::from([
            (Token::Equal.token_type(), Priority::Equals),
            (Token::NotEqual.token_type(), Priority::Equals),
            (Token::Lt.token_type(), Priority::LessGreater),
            (Token::Gt.token_type(), Priority::LessGreater),
            (Token::Plus.token_type(), Priority::Sum),
            (Token::Minus.token_type(), Priority::Sum),
            (Token::Divide.token_type(), Priority::Product),
            (Token::Multiply.token_type(), Priority::Product),
        ]);
        let mut p = Parser {
            lexer,
            curr_token: None,
            peek_token: None,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
            precedences
        };
        p.next_token();
        p.next_token();
        p.register_infix_fns();
        p.register_prefix_fns();
        p
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.take();
        let tok = self.lexer.next_token();
        self.peek_token = Some(tok);
    }

    pub fn parse_program(&mut self) -> Result<Program, Box<dyn Error>> {
        let mut program = Program::new();
        while let Some(token) = &self.curr_token {
            match token {
                Token::Eof => break,
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

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn curr_precedence(&self) -> Priority {
        if let Some(v) = self.precedences.get(
            &self.curr_token
                .clone()
                .expect("could not peek precendence becasuse token was none")
                .token_type(),
        ) {
            return *v;
        }
        Priority::Lowest
    }

    fn peek_precedence(&self) -> Priority {
        if let Some(v) = self.precedences.get(
            &self.peek_token
                .clone()
                .expect("could not peek precendence becasuse token was none")
                .token_type(),
        ) {
            return *v;
        }
        Priority::Lowest
    }

    fn register_infix_fns(&mut self) {
        self.register_infix(
            Token::Plus.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Minus.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Divide.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Multiply.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Equal.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::NotEqual.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Lt.token_type(), Parser::parse_infix_expression
        );
        self.register_infix(
            Token::Gt.token_type(), Parser::parse_infix_expression
        );
    }

    fn register_prefix_fns(&mut self) {
        self.register_prefix(
            Token::Ident("".into()).token_type(),
            Parser::parse_identifier,
        );
        self.register_prefix(
            Token::Integer("1".into()).token_type(),
            Parser::parse_integer_literal,
        );
        self.register_prefix(Token::Bang.token_type(), Parser::parse_prefix_expression);
        self.register_prefix(Token::Minus.token_type(), Parser::parse_prefix_expression);
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let curr_token = &self.curr_token;
        match curr_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(_) => self.parse_expression_statement(),
            _ => None,
        }
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let return_token = self.curr_token.clone();
        if return_token.is_none() {
            return None;
        }
        let return_token = return_token.unwrap();
        self.next_token();
        let break_token = discriminant(&Token::Semicolon);
        while let Some(tok) = &self.curr_token {
            if discriminant::<Token>(&tok) == break_token {
                break;
            } else {
                self.next_token();
            }
        }
        // Some(Box::new(ReturnStatement::new(return_token, expression)))
        todo!()
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let let_token = self.curr_token.take();
        if let_token.is_none() {
            return None;
        }
        let let_token = let_token.as_ref().unwrap();

        let exp_token = Token::Ident("".into());
        if !&self.expect_peek(exp_token) {
            return None;
        }
        let ident_token = self.curr_token.take();
        if ident_token.is_none() {
            return None;
        }
        let ident_token = ident_token.unwrap();
        let ident_literal = match &ident_token {
            Token::Ident(value) => value.clone(),
            _ => unreachable!(),
        };
        let ident = Identifier::new(ident_token, ident_literal);

        let exp_token = Token::Assign;
        if !&self.expect_peek(exp_token) {
            return None;
        }
        let break_token = discriminant(&Token::Semicolon);
        while let Some(tok) = &self.curr_token {
            if discriminant::<Token>(&tok) == break_token {
                break;
            } else {
                self.next_token();
            }
        }
        // Some(Box::new(LetStatement::new(let_token, ident)))
        todo!()
    }

    fn expect_peek(&mut self, token_type: Token) -> bool {
        match &self.peek_token {
            Some(tok) if discriminant::<Token>(&tok) == discriminant::<Token>(&token_type) => {
                self.next_token();
                true
            }
            _ => {
                self.peek_error(token_type);
                false
            }
        }
    }

    fn peek_error(&mut self, token: Token) {
        let tok = &self.peek_token;
        let msg = format!(
            "expected next token to be {:?}, but received {:?}",
            token,
            tok.as_ref().unwrap_or(&Token::Illegal)
        );
        self.errors.push(msg);
    }

    fn register_prefix(&mut self, token_type: String, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: String, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let expression = self
            .parse_expression(Priority::Lowest)
            .expect("Should never be None");

        let stmt = ExpressionStatement::new(
            self.curr_token.take().expect("Should never be None"),
            expression,
        );

        let _b = self.expect_peek(Token::Semicolon);
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Priority) -> Option<Box<dyn Expression>> {
        if self.curr_token.is_none() {
            return None;
        }
        let token_type = self.curr_token.as_mut().unwrap().token_type();
        let prefix_func = self
            .prefix_parse_fns
            .get(&token_type)
            .expect(format!("Could not find function with key: {}", token_type).as_str());
        let mut left_exp = prefix_func(self);

        while self.peek_token != Some(Token::Semicolon) && precedence < self.peek_precedence() {
            if self.peek_token.is_none() {
                return Some(left_exp);
            }
            let peek_token_type = self.peek_token.as_mut().unwrap().token_type();
            let infix_func = self
                .infix_parse_fns
                .get(&peek_token_type)
                .expect(format!("Could not find function with key: {}", peek_token_type).as_str())
                .clone();
            self.next_token();
            left_exp = infix_func(self,left_exp);
        }
        Some(left_exp)
    }

    fn parse_identifier(&mut self) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point");
        let literal = tok.literal();
        Box::new(Identifier::new(tok.clone(), literal.into()))
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point");
        let literal = tok
            .literal()
            .parse::<isize>()
            .expect("Type was not a number");
        Box::new(IntegerLiteral::new(tok.clone(), literal))
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point")
            .clone();
        let literal = String::from(tok.literal());
        self.next_token();
        let expression_right = self
            .parse_expression(Priority::Prefix)
            .expect("Failed to parse right side of prefix expression");
        Box::new(PrefixExpression::new(tok, literal, expression_right))
    }

    fn parse_infix_expression(&mut self, expression_left: Box<dyn Expression>) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point")
            .clone();
        let operator = String::from(tok.literal());
        let precedence = self.curr_precedence();
        self.next_token();
        let expression_right = self
            .parse_expression(precedence)
            .expect("Failed to parse right side of infix expression");
        Box::new(InfixExpression::new(tok, operator, expression_right, expression_left))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression_statement::ExpressionStatement;
    use crate::ast::infix_expression::InfixExpression;
    use crate::ast::integer_literal::IntegerLiteral;
    use crate::ast::let_statement::LetStatement;
    use crate::ast::prefix_expression::PrefixExpression;
    use crate::ast::Node;

    fn test_helper(input: &str) -> Program {
        let l = Lexer::new(input.into());
        let mut parser = Parser::new(l);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert!(program.is_ok());
        program.unwrap()
    }

    fn check_parser_errors(p: &Parser) {
        if p.errors().len() == 0 {
            return;
        }
        println!("Parser had {} errors", p.errors.len());
        for msg in p.errors() {
            println!("parser error: {}", msg);
        }
        panic!();
    }

    fn test_integer(expression: &IntegerLiteral, integer: isize) {
        assert_eq!(&integer, expression.value());
        assert_eq!(format!("{}", integer), expression.token_literal());
    }

    #[test]
    #[ignore]
    fn test_let_statement() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let program = test_helper(input);
        assert_eq!(program.statements.len(), 3);

        let expexted_identifiers = vec!["x", "y", "foobar"];

        for (i, ident) in expexted_identifiers.iter().enumerate() {
            let stmt = &program.statements[i];
            assert_eq!(stmt.token_literal(), "let");
            let stmt: &LetStatement = match stmt.as_any().downcast_ref::<LetStatement>() {
                Some(b) => b,
                None => panic!("Could not cast statement to let statement"),
            };
            assert_eq!(stmt.name().value(), *ident);
            assert_eq!(stmt.name().token_literal(), *ident);
        }
    }

    #[test]
    #[should_panic]
    fn test_let_statement_errors() {
        let input = r#"
        let x = 5;
        let y = 10;
        let  838383;
        "#;

        let _ = test_helper(input);
    }

    #[test]
    #[ignore]
    fn test_return_statement() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;
        let program = test_helper(input);
        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements {
            assert_eq!(stmt.token_literal(), "return");
        }
    }

    #[test]
    fn test_ident_expression() {
        let input = "foobar;";
        let program = test_helper(input);
        assert_eq!(1, program.statements.len());
        let statement = program.statements.get(0).unwrap().as_any();
        let statement = match statement.downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Statement was not an expression statement"),
        };
        let ident = match statement.expression().as_any().downcast_ref::<Identifier>() {
            Some(v) => v,
            None => panic!("Statement was not an identifier"),
        };
        assert_eq!("foobar", ident.value());
        assert_eq!("foobar", ident.token_literal());
    }

    #[test]
    fn test_integer_expression() {
        let input = "5;";
        let program = test_helper(input);
        assert_eq!(1, program.statements.len());
        let statement = program.statements.get(0).unwrap();
        let statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Statement was not an expression statement"),
        };
        let integer = match statement
            .expression()
            .as_any()
            .downcast_ref::<IntegerLiteral>()
        {
            Some(v) => v,
            None => panic!("Statement was not an IntegerLiteral"),
        };
        test_integer(integer, 5);
    }

    #[test]
    fn test_prefix_expressions() {
        let v = vec![("!5;", "!", 5), ("-15;", "-", 15)];
        for exp in v {
            let program = test_helper(exp.0);
            let statement = program.statements.get(0).unwrap().as_any();
            let statement = match statement.downcast_ref::<ExpressionStatement>() {
                Some(v) => v,
                None => panic!("Statement was not an expression statement"),
            };
            let prefix = match statement
                .expression()
                .as_any()
                .downcast_ref::<PrefixExpression>()
            {
                Some(v) => v,
                None => panic!("Statement was not a Prefix Expression"),
            };
            assert_eq!(prefix.operator(), exp.1);
            let integer = match prefix
                .expression_right()
                .as_any()
                .downcast_ref::<IntegerLiteral>()
            {
                Some(v) => v,
                None => panic!("Statement was not an Integer Literal"),
            };
            test_integer(integer, exp.2);
        }
    }

    #[test]
    fn test_infix_expressions() {
        let v = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];
        for exp in v {
            let program = test_helper(exp.0);
            let statement = program.statements.get(0).unwrap().as_any();
            let statement = match statement.downcast_ref::<ExpressionStatement>() {
                Some(v) => v,
                None => panic!("Statement was not an expression statement"),
            };
            let infix = match statement
                .expression()
                .as_any()
                .downcast_ref::<InfixExpression>()
            {
                Some(v) => v,
                None => panic!("Statement was not a Infix Expression"),
            };
            let integer = match infix
                .expression_left()
                .as_any()
                .downcast_ref::<IntegerLiteral>()
            {
                Some(v) => v,
                None => panic!("Statement was not an Integer Literal"),
            };
            test_integer(integer, exp.1);
            let integer = match infix
                .expression_right()
                .as_any()
                .downcast_ref::<IntegerLiteral>()
            {
                Some(v) => v,
                None => panic!("Statement was not an Integer Literal"),
            };
            test_integer(integer, exp.3);
        }
    }
}
