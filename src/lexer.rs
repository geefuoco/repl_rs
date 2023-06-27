use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Default, PartialOrd)]
pub enum Token {
    Ident(String),
    Integer(String),
    String(String),
    Function,
    Let,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lsquare,
    Rsquare,
    Comma,
    Semicolon,
    Assign,
    Plus,
    Minus,
    Lt,
    Gt,
    Equal,
    And,
    Or,
    NotEqual,
    If,
    Else,
    Return,
    True,
    False,
    Divide,
    Multiply,
    Bang,
    SingleQuote,
    Eof,
    #[default]
    Illegal,
}

impl Token {
    pub fn literal(&self) -> &str {
        match self {
            Token::Ident(v) => v,
            Token::Integer(v) => v,
            Token::String(v) => v,
            Token::Function => "fn",
            Token::Let => "let",
            Token::Lparen => "(",
            Token::Rparen => ")",
            Token::Lbrace => "{",
            Token::Rbrace => "}",
            Token::Lsquare => "[",
            Token::Rsquare => "]",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::Equal => "==",
            Token::And => "&",
            Token::Or => "|",
            Token::NotEqual => "!=",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",
            Token::Divide => "/",
            Token::Multiply => "*",
            Token::Bang => "!",
            Token::SingleQuote => "'",
            Token::Eof => "EOF",
            Token::Illegal => "Illegal",
        }
    }

    pub fn token_type(&self) -> String {
        match self {
            Token::Ident(_) => String::from("Ident"),
            Token::Integer(_) => String::from("Integer"),
            Token::String(_) => String::from("String"),
            Token::Function => String::from("Function"),
            Token::Let => String::from("Let"),
            Token::Lparen => String::from("Lparen"),
            Token::Rparen => String::from("Rparen"),
            Token::Lbrace => String::from("Lbrace"),
            Token::Rbrace => String::from("Rbrace"),
            Token::Lsquare => String::from("Lsquare"),
            Token::Rsquare => String::from("Rsquare"),
            Token::Comma => String::from("Comma"),
            Token::Semicolon => String::from("Semicolon"),
            Token::Assign => String::from("Assign"),
            Token::Plus => String::from("Plus"),
            Token::Minus => String::from("Minus"),
            Token::Lt => String::from("Lt"),
            Token::Gt => String::from("Gt"),
            Token::Equal => String::from("Equal"),
            Token::And => String::from("And"),
            Token::Or => String::from("Or"),
            Token::NotEqual => String::from("NotEqual"),
            Token::If => String::from("If"),
            Token::Else => String::from("Else"),
            Token::Return => String::from("Return"),
            Token::True => String::from("True"),
            Token::False => String::from("False"),
            Token::Divide => String::from("Divide"),
            Token::Multiply => String::from("Multiply"),
            Token::Bang => String::from("Bang"),
            Token::SingleQuote => String::from("SingleQuote"),
            Token::Eof => String::from("Eof"),
            Token::Illegal => String::from("Illegal"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        write!(f, ": {}", self.literal())
    }
}

#[derive(Debug)]
pub struct Lexer {
    read_position: usize,
    position: usize,
    input: Vec<u8>,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            position: 0,
            read_position: 0,
            input: input.into_bytes(),
            ch: 0,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let tok = match ident {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Ident(ident.into()),
                };
                return tok;
            }
            b'0'..=b'9' => {
                let ident = self.read_int();
                let tok = Token::Integer(ident.into());
                return tok;
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'[' => Token::Lsquare,
            b']' => Token::Rsquare,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'"' => {
                self.read_char();
                return Token::String(self.read_str().into());
            }
            b'=' => match self.peek() {
                b'=' => {
                    self.read_char();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => match self.peek() {
                b'=' => {
                    self.read_char();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'\'' => Token::SingleQuote,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'&' => Token::And,
            b'|' => Token::Or,
            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' && self.ch != 0 {
            self.read_char();
        }
        match std::str::from_utf8(&self.input[pos..self.position]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd"),
        }
    }

    fn read_int(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_ascii_digit() && self.ch != 0 {
            self.read_char();
        }
        match std::str::from_utf8(&self.input[pos..self.position]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd"),
        }
    }

    fn peek(&mut self) -> &u8 {
        if self.read_position < self.input.len() {
            return &self.input[self.read_position];
        }
        &0
    }

    fn read_str(&mut self) -> &str {
        let pos = self.position;
        while &self.ch != &b'"' && self.ch != 0 {
            self.read_char();
        }
        self.read_char();
        match std::str::from_utf8(&self.input[pos..self.position - 1]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l = Lexer::new("=+(){},;".into());

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            assert_eq!(token, l.next_token());
        }
    }

    #[test]
    fn more_complex_example() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let string = "hello";
        let ch = 'a';
        let result = add(five, ten);"#;

        let mut l = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("string".into()),
            Token::Assign,
            // Token::DoubleQuote,
            Token::String("hello".into()),
            // Token::DoubleQuote,
            Token::Semicolon,
            Token::Let,
            Token::Ident("ch".into()),
            Token::Assign,
            Token::SingleQuote,
            Token::Ident("a".into()),
            Token::SingleQuote,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            assert_eq!(token, l.next_token());
        }
    }

    #[test]
    fn logical_operators() {
        let input = r#"
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let mut l = Lexer::new(input.into());

        let tokens = vec![
            Token::Bang,
            Token::Minus,
            Token::Divide,
            Token::Multiply,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Integer("5".into()),
            Token::Lt,
            Token::Integer("10".into()),
            Token::Gt,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Integer("5".into()),
            Token::Lt,
            Token::Integer("10".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Integer("10".into()),
            Token::Equal,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Integer("10".into()),
            Token::NotEqual,
            Token::Integer("9".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            assert_eq!(token, l.next_token());
        }
    }

    #[test]
    fn strings_only() {
        let input = r#"
        let x =      "hello"; 
        let y ="world!"; 
        let test = "'abc123_!@#$%^&*(){};";
        "#;

        let mut l = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("x".into()),
            Token::Assign,
            // Token::DoubleQuote,
            Token::String("hello".into()),
            // Token::DoubleQuote,
            Token::Semicolon,
            Token::Let,
            Token::Ident("y".into()),
            Token::Assign,
            // Token::DoubleQuote,
            Token::String("world!".into()),
            // Token::DoubleQuote,
            Token::Semicolon,
            Token::Let,
            Token::Ident("test".into()),
            Token::Assign,
            // Token::DoubleQuote,
            Token::String("'abc123_!@#$%^&*(){};".into()),
            // Token::DoubleQuote,
            Token::Semicolon,
        ];

        for token in tokens {
            assert_eq!(token, l.next_token());
        }
    }
}
