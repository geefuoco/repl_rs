use crate::ast::BlockStatement;
use crate::ast::BooleanLiteral;
use crate::ast::ExpressionStatement;
use crate::ast::FunctionLiteral;
use crate::ast::IfExpression;
use crate::ast::InfixExpression;
use crate::ast::IntegerLiteral;
use crate::ast::OptionalBlockStatement;
use crate::ast::PrefixExpression;
use crate::ast::ReturnStatement;
use crate::{
    ast::{AsAny, Expression, Identifier, Program, Statement},
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
            precedences,
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
            &self
                .curr_token
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
            &self
                .peek_token
                .clone()
                .expect("could not peek precendence becasuse token was none")
                .token_type(),
        ) {
            return *v;
        }
        Priority::Lowest
    }

    fn register_infix_fns(&mut self) {
        self.register_infix(Token::Plus.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Minus.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Divide.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Multiply.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Equal.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::NotEqual.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Lt.token_type(), Parser::parse_infix_expression);
        self.register_infix(Token::Gt.token_type(), Parser::parse_infix_expression);
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
        self.register_prefix(Token::True.token_type(), Parser::parse_boolean);
        self.register_prefix(Token::False.token_type(), Parser::parse_boolean);
        self.register_prefix(Token::Lparen.token_type(), Parser::parse_grouped_expression);
        self.register_prefix(Token::If.token_type(), Parser::parse_if_expression);
        self.register_prefix(Token::Function.token_type(), Parser::parse_function_literal);
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

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        };
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
            left_exp = infix_func(self, left_exp);
        }
        Some(left_exp)
    }

    fn parse_if_expression(&mut self) -> Box<dyn Expression> {
        match self.expect_peek(Token::Lparen) {
            true => {
                let curr_token = self
                    .curr_token
                    .clone()
                    .expect("Could not find current token");
                self.next_token();
                let condition = self
                    .parse_expression(Priority::Lowest)
                    .expect("Could not parse expression");
                if !self.expect_peek(Token::Rparen) {
                    panic!("Missing Right Parenthesis");
                }
                if !self.expect_peek(Token::Lbrace) {
                    panic!("Missing Left Brace");
                }
                let consequence = self.parse_block_statement();
                let mut alternative = OptionalBlockStatement::new(None);
                if self.peek_token == Some(Token::Else) {
                    self.next_token();
                    if !self.expect_peek(Token::Lbrace) {
                        panic!("Missing Left Brace");
                    }
                    alternative = OptionalBlockStatement::new(Some(self.parse_block_statement()));
                }
                Box::new(IfExpression::new(
                    curr_token,
                    condition,
                    consequence,
                    alternative,
                ))
            }
            false => panic!("Missing Left Parenthesis"),
        }
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut v = Vec::new();
        let curr_token = self
            .curr_token
            .clone()
            .expect("Could not find current token");
        self.next_token();
        while self.curr_token != Some(Token::Rbrace) && self.curr_token != Some(Token::Eof) {
            let stmt = self.parse_statement();
            match stmt {
                Some(stmt) => v.push(stmt),
                None => {}
            }
            self.next_token();
        }
        BlockStatement::new(curr_token, v)
    }

    fn parse_grouped_expression(&mut self) -> Box<dyn Expression> {
        self.next_token();
        let exp = self.parse_expression(Priority::Lowest);
        match self.expect_peek(Token::Rparen) {
            true => exp.expect("Missing Right Parenthesis"),
            false => panic!("Missing Right Parenthesis"),
        }
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

    fn parse_boolean(&mut self) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point");
        let literal = tok.literal().parse::<bool>().expect("Type was not a bool");
        Box::new(BooleanLiteral::new(tok.clone(), literal))
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

    fn parse_infix_expression(
        &mut self,
        expression_left: Box<dyn Expression>,
    ) -> Box<dyn Expression> {
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
        Box::new(InfixExpression::new(
            tok,
            operator,
            expression_left,
            expression_right,
        ))
    }

    fn parse_function_literal(&mut self) -> Box<dyn Expression> {
        let tok = self
            .curr_token
            .as_ref()
            .expect("should exist at this point")
            .clone();
        if !self.expect_peek(Token::Lparen) {
            panic!("Missing Left Parenthesis");
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(Token::Lbrace) {
            panic!("Missing Left Brace");
        }
        let body = self.parse_block_statement();
        Box::new(FunctionLiteral::new(tok, parameters, body))
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut v = Vec::new();
        if self.peek_token == Some(Token::Rparen) {
            self.next_token();
            return v;
        }
        self.next_token();
        let tok = self.curr_token.as_ref().expect("should be present");
        let literal = tok.literal();
        let ident = Identifier::new(tok.clone(), literal.to_string());
        v.push(ident);

        while self.peek_token == Some(Token::Comma) {
            self.next_token();
            self.next_token();
            let tok = self.curr_token.as_ref().expect("should be present");
            let literal = tok.literal();
            let ident = Identifier::new(tok.clone(), literal.to_string());
            v.push(ident);
        }

        if !self.expect_peek(Token::Rparen) {
            panic!("Missing Right parenthesis");
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::block_statement::BlockStatement;
    use crate::ast::boolean_literal::BooleanLiteral;
    use crate::ast::expression_statement::ExpressionStatement;
    use crate::ast::if_expression::IfExpression;
    use crate::ast::infix_expression::InfixExpression;
    use crate::ast::integer_literal::IntegerLiteral;
    use crate::ast::let_statement::LetStatement;
    use crate::ast::prefix_expression::PrefixExpression;
    use crate::ast::{FunctionLiteral, Node};

    enum Types<'a> {
        String(&'a str),
        Isize(isize),
        Bool(bool),
    }

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

    fn test_ident(expression: &Identifier, value: &str) {
        assert_eq!(value, expression.value());
        assert_eq!(value, expression.token_literal());
    }

    fn test_bool(expression: &BooleanLiteral, boolean: bool) {
        assert_eq!(&boolean, expression.value());
        assert_eq!(format!("{}", boolean), expression.token_literal());
    }

    fn test_literal(expression: &Box<impl Expression + ?Sized>, value: Types) {
        match value {
            Types::String(x) => {
                println!("{}", expression);
                let expression = match expression.as_any().downcast_ref::<Identifier>() {
                    Some(v) => v,
                    None => panic!("Could not convert expression to Identifier"),
                };
                test_ident(expression, &x);
            }
            Types::Isize(x) => {
                println!("{}", expression);
                let expression = match expression.as_any().downcast_ref::<IntegerLiteral>() {
                    Some(v) => v,
                    None => panic!("Could not convert expression to IntegerLiteral"),
                };
                test_integer(expression, x);
            }
            Types::Bool(x) => {
                println!("{}", expression);
                let expression = match expression.as_any().downcast_ref::<BooleanLiteral>() {
                    Some(v) => v,
                    None => panic!("Could not convert expression to BooleanLiteral"),
                };
                test_bool(expression, x);
            }
        }
    }

    fn test_infix_expression(expression: &InfixExpression, left: Types, op: &str, right: Types) {
        match left {
            Types::String(x) => test_literal(expression.expression_left(), Types::String(x)),
            Types::Isize(x) => test_literal(expression.expression_left(), Types::Isize(x)),
            Types::Bool(x) => test_literal(expression.expression_left(), Types::Bool(x)),
        }
        match right {
            Types::String(x) => test_literal(expression.expression_right(), Types::String(x)),
            Types::Isize(x) => test_literal(expression.expression_right(), Types::Isize(x)),
            Types::Bool(x) => test_literal(expression.expression_right(), Types::Bool(x)),
        }

        let expression = match expression.as_any().downcast_ref::<InfixExpression>() {
            Some(v) => v,
            None => panic!("Could not convert expression to InfixExpression"),
        };

        assert_eq!(expression.operator(), op);
    }

    #[test]
    fn test_new_helper() {
        let exp = &InfixExpression::new(
            Token::Integer(String::from("5")),
            String::from("+"),
            Box::new(IntegerLiteral::new(Token::Integer(String::from("5")), 5)),
            Box::new(IntegerLiteral::new(Token::Integer(String::from("10")), 10)),
        );
        test_infix_expression(exp, Types::Isize(5), "+", Types::Isize(10));
        let binding = &InfixExpression::new(
            Token::Ident(String::from("foo")),
            String::from("*"),
            Box::new(Identifier::new(
                Token::Ident(String::from("foo")),
                "foo".into(),
            )),
            Box::new(Identifier::new(
                Token::Ident(String::from("bar")),
                "bar".into(),
            )),
        );
        let exp = Box::new(binding);
        test_infix_expression(&exp, Types::String("foo"), "*", Types::String("bar"));
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
        let mut program = test_helper(input);
        assert_eq!(1, program.statements.len());
        let statement = program.statements.remove(0);
        println!("{:?}", statement.type_id());
        let statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Statement was not an expression statement"),
        };
        let ident = match statement.expression().as_any().downcast_ref::<Identifier>() {
            Some(v) => v,
            None => panic!("Statement was not an identifier"),
        };
        test_ident(ident, &input[..input.len() - 1]);
    }

    #[test]
    fn test_boolean_expression() {
        let input = "false;";
        let program = test_helper(input);
        assert_eq!(1, program.statements.len());
        let statement = program.statements.get(0).unwrap();
        let statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Statement was not an expression statement"),
        };
        let boolean = match statement
            .expression()
            .as_any()
            .downcast_ref::<BooleanLiteral>()
        {
            Some(v) => v,
            None => panic!("Statement was not an BooleanLiteral"),
        };
        test_bool(boolean, false);
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
    fn test_prefix_expressions_bools() {
        let v = vec![("!true;", "!", true), ("!false;", "!", false)];
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
                .downcast_ref::<BooleanLiteral>()
            {
                Some(v) => v,
                None => panic!("Statement was not an Integer Literal"),
            };
            test_bool(integer, exp.2);
        }
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
    fn test_infix_expressions_bools() {
        let v = vec![
            ("true == true", true, "==", true),
            ("true != false", true, "!=", false),
            ("false == false", false, "==", false),
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
                .clone()
                .as_mut()
            {
                Some(v) => v.to_owned(),
                None => panic!("Statement was not a Infix Expression"),
            };
            test_infix_expression(
                &Box::new(infix),
                Types::Bool(exp.1),
                exp.2,
                Types::Bool(exp.3),
            );
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
                .clone()
                .as_mut()
            {
                Some(v) => v.to_owned(),
                None => panic!("Statement was not a Infix Expression"),
            };
            test_infix_expression(
                &Box::new(infix),
                Types::Isize(exp.1),
                exp.2,
                Types::Isize(exp.3),
            );
        }
    }

    #[test]
    fn test_operator_precedence_as_string() {
        let input_expected = [
            ("-a * b", "((-a) * b)\n"),
            ("!-a", "(!(-a))\n"),
            ("a + b + c", "((a + b) + c)\n"),
            ("a + b + c", "((a + b) + c)\n"),
            ("a + b - c", "((a + b) - c)\n"),
            ("a * b * c", "((a * b) * c)\n"),
            ("a * b / c", "((a * b) / c)\n"),
            ("a + b / c", "(a + (b / c))\n"),
            ("a * b + c", "((a * b) + c)\n"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)\n"),
            ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)\n"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))\n"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))\n"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\n",
            ),
            (
                "3 + 4 * 5 != 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) != ((3 * 1) + (4 * 5)))\n",
            ),
            ("true", "true\n"),
            ("false", "false\n"),
            ("3 > 5 == false", "((3 > 5) == false)\n"),
            ("3 < 5 == true", "((3 < 5) == true)\n"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)\n"),
            ("(5 + 5) * 2", "((5 + 5) * 2)\n"),
            ("2 / (5 + 5)", "(2 / (5 + 5))\n"),
            ("-(5 + 5)", "(-(5 + 5))\n"),
            ("!(true == true)", "(!(true == true))\n"),
        ];

        for input in input_expected {
            let program = test_helper(input.0);
            assert_eq!(input.1, format!("{}", program));
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let program = test_helper(input);
        let statement = program.statements.get(0).expect("Could not find statement");
        let statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Could not convert into ExpressionStatement"),
        };
        let if_expression = match statement
            .expression()
            .as_any()
            .downcast_ref::<IfExpression>()
        {
            Some(v) => v,
            None => panic!("Could not convert into IfExpression"),
        };
        let condition = match if_expression
            .condition()
            .as_any()
            .downcast_ref::<InfixExpression>()
        {
            Some(v) => v,
            None => panic!("Could not convert into InfixExpression"),
        };
        test_infix_expression(
            &Box::new(condition),
            Types::String("x"),
            "<",
            Types::String("y"),
        );
        let consequence = if_expression
            .consequence()
            .as_any()
            .downcast_ref::<BlockStatement>()
            .expect("Could not convert into BlockStatement")
            .statements()
            .get(0)
            .expect("Could not get statement")
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Could not convert into ExpressionStatement");
        let ident = consequence
            .expression()
            .as_any()
            .downcast_ref::<Identifier>()
            .expect("Could not convert into ExpressionStatement");
        test_ident(ident, "x");
        assert!(if_expression.alternative().is_none());
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let program = test_helper(input);
        let statement = program.statements.get(0).expect("Could not find statement");
        let statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("Could not convert into ExpressionStatement"),
        };
        let if_expression = match statement
            .expression()
            .as_any()
            .downcast_ref::<IfExpression>()
        {
            Some(v) => v,
            None => panic!("Could not convert into IfExpression"),
        };
        let condition = match if_expression
            .condition()
            .as_any()
            .downcast_ref::<InfixExpression>()
        {
            Some(v) => v,
            None => panic!("Could not convert into InfixExpression"),
        };
        test_infix_expression(
            &Box::new(condition),
            Types::String("x"),
            "<",
            Types::String("y"),
        );
        let consequence = if_expression
            .consequence()
            .as_any()
            .downcast_ref::<BlockStatement>()
            .expect("Could not convert into block statement")
            .statements()
            .get(0)
            .expect("Could not get statement")
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Could not convert into ExpressionStatement");
        let ident = consequence
            .expression()
            .as_any()
            .downcast_ref::<Identifier>()
            .expect("Could not convert into Identifier");
        test_ident(ident, "x");
        let alternative = if_expression
            .alternative()
            .as_ref()
            .expect("Expected there to be an else block")
            .as_any()
            .downcast_ref::<BlockStatement>()
            .expect("Could not convert to BlockStatement")
            .statements()
            .get(0)
            .expect("Could not get statement")
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Could not convert into ExpressionStatement")
            .expression()
            .as_any()
            .downcast_ref::<Identifier>()
            .expect("Could not convert into Identifier");
        test_ident(alternative, "y");
    }

    #[test]
    fn test_fn_literal() {
        let input = "fn(x, y) { x + y }";
        let program = test_helper(input);
        let statement = program.statements.get(0).unwrap();
        let exp_statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(v) => v,
            None => panic!("statement was not an ExpressionStatement"),
        };
        let fn_literal = match exp_statement
            .expression()
            .as_any()
            .downcast_ref::<FunctionLiteral>()
        {
            Some(v) => v,
            None => panic!("expression was not a FunctionLiteral"),
        };
        assert_eq!(fn_literal.parameters().len(), 2);
        test_ident(fn_literal.parameters().get(0).unwrap(), "x");
        test_ident(fn_literal.parameters().get(1).unwrap(), "y");
        assert_eq!(fn_literal.body().statements().len(), 1);
        let infix = match fn_literal
            .body()
            .statements()
            .get(0)
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Should be an expression statement i hope")
            .expression()
            .as_any()
            .downcast_ref::<InfixExpression>()
        {
            Some(v) => v,
            None => panic!("fn_literal body statement was not an InfixExpression"),
        };
        test_infix_expression(infix, Types::String("x"), "+", Types::String("y"));
    }

    #[test]
    fn test_function_parameter_parsing() {
        let inputs = [
            ("fn() {};", vec![]),
            ("fn(x) {};", vec!["x"]),
            ("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];

        for (input, expected) in inputs {
            let program = test_helper(input);
            let statement = program.statements.get(0).unwrap();
            let exp_statement = match statement.as_any().downcast_ref::<ExpressionStatement>() {
                Some(v) => v,
                None => panic!("statement was not an ExpressionStatement"),
            };
            let fn_literal = match exp_statement
                .expression()
                .as_any()
                .downcast_ref::<FunctionLiteral>()
            {
                Some(v) => v,
                None => panic!("ExpressionStatement was not FunctionLiteral"),
            };

            assert_eq!(expected.len(), fn_literal.parameters().len());
            for (i, ident) in fn_literal.parameters().iter().enumerate() {
                test_ident(ident, expected.get(i).unwrap());
            }
        }
    }
}
