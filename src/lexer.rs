use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IDENT(String),
    INTEGER(String),
    STRING(String),
    FUNCTION,
    LET,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LSQUARE,
    RSQUARE,
    COMMA,
    SEMICOLON,
    ASSIGN,
    PLUS,
    MINUS,
    LT,
    GT,
    EQUAL,
    AND,
    OR,
    NOT_EQUAL,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    DIVIDE,
    MULTIPLY,
    BANG,
    SINGLE_QUOTE,
    DOUBLE_QUOTE,
    EOF,
    ILLEGAL
}

impl Token {
    pub fn literal(&self) -> &str{
        match self {
            Token::IDENT(v) =>v,
            Token::INTEGER(v) =>v,
            Token::STRING(v) =>v,
            Token::FUNCTION =>"fn".into(),
            Token::LET =>"let".into(),
            Token::LPAREN =>"(".into(),
            Token::RPAREN =>")".into(),
            Token::LBRACE =>"{".into(),
            Token::RBRACE =>"}".into(),
            Token::LSQUARE =>"[".into(),
            Token::RSQUARE =>"]".into(),
            Token::COMMA =>",".into(),
            Token::SEMICOLON =>";".into(),
            Token::ASSIGN =>"=".into(),
            Token::PLUS =>"+".into(),
            Token::MINUS =>"-".into(),
            Token::LT =>"<".into(),
            Token::GT =>">".into(),
            Token::EQUAL =>"==".into(),
            Token::AND =>"&".into(),
            Token::OR =>"|".into(),
            Token::NOT_EQUAL =>"!=".into(),
            Token::IF =>"if".into(),
            Token::ELSE =>"else".into(),
            Token::RETURN =>"return".into(),
            Token::TRUE =>"true".into(),
            Token::FALSE =>"false".into(),
            Token::DIVIDE =>"/".into(),
            Token::MULTIPLY =>"*".into(),
            Token::BANG =>"!".into(),
            Token::SINGLE_QUOTE =>"'".into(),
            Token::DOUBLE_QUOTE =>"\"".into(),
            Token::EOF =>"EOF".into(),
            Token::ILLEGAL =>"ILLEGAL".into(),
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
    parsing_string: bool,
    prev_token_string: bool,
}


impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            position: 0,
            read_position: 0,
            input: input.into_bytes(),
            ch: 0,
            parsing_string: false,
            prev_token_string: false,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.parsing_string {
            let value = self.read_str();
            return Token::STRING(value.into());
        }
        let tok = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let tok = match ident {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    "if" => Token::IF,
                    "else" => Token::ELSE,
                    "return" => Token::RETURN,
                    "true" => Token::TRUE,
                    "false" => Token::FALSE,
                    _ => Token::IDENT(ident.into())
                };
                return tok;
            },
            b'0'..=b'9' => {
                let ident = self.read_int();
                let tok = Token::INTEGER(ident.into());
                return tok;
            },
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'[' => Token::LSQUARE,
            b']' => Token::RSQUARE,
            b';' => Token::SEMICOLON,
            b',' => Token::COMMA,
            b'"' => {
                if !self.prev_token_string {
                    self.parsing_string = true;
                } else {
                    self.prev_token_string = false;
                }
                Token::DOUBLE_QUOTE
            },
            b'=' => {
                match self.peek() {
                    b'=' => {
                        self.read_char();
                        Token::EQUAL
                    },
                    _ => Token::ASSIGN,
                }
                
            },
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' =>{ 
                match self.peek() {
                    b'=' => {
                        self.read_char();
                        Token::NOT_EQUAL
                    },
                    _ => Token::BANG,
                }
            },
            b'*' => Token::MULTIPLY,
            b'/' => Token::DIVIDE,
            b'\'' => Token::SINGLE_QUOTE,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'&' => Token::AND,
            b'|' => Token::OR,
            0 => Token::EOF,
            _ => Token::ILLEGAL
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

    fn read_ident(&mut self) -> &str{
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' && self.ch != 0 {
            self.read_char();
        } 
        match std::str::from_utf8(&self.input[pos..self.position]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd")
        }
    }

    fn read_int(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_ascii_digit() && self.ch != 0 {
            self.read_char();
        } 
        match std::str::from_utf8(&self.input[pos..self.position]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd")
        }
    }

    fn peek(&mut self) -> &u8 {
        if self.read_position < self.input.len() {
            return &self.input[self.read_position]
        }
        &0
    }

    fn read_str(&mut self) -> &str {
        let pos = self.position;
        while &self.ch != &b'"' && self.ch != 0 {
            self.read_char();
        }
        self.parsing_string = false;
        self.prev_token_string = true;
        match std::str::from_utf8(&self.input[pos..self.position]) {
            Ok(valid) => valid,
            _ => panic!("Non UTF-8 Character encounterd")
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
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
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
            Token::LET,
            Token::IDENT("five".into()),
            Token::ASSIGN,
            Token::INTEGER("5".into()),
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("ten".into()),
            Token::ASSIGN,
            Token::INTEGER("10".into()),
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("add".into()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".into()),
            Token::COMMA,
            Token::IDENT("y".into()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".into()),
            Token::PLUS,
            Token::IDENT("y".into()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("string".into()),
            Token::ASSIGN,
            Token::DOUBLE_QUOTE,
            Token::STRING("hello".into()),
            Token::DOUBLE_QUOTE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("ch".into()),
            Token::ASSIGN,
            Token::SINGLE_QUOTE,
            Token::IDENT("a".into()),
            Token::SINGLE_QUOTE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("result".into()),
            Token::ASSIGN,
            Token::IDENT("add".into()),
            Token::LPAREN,
            Token::IDENT("five".into()),
            Token::COMMA,
            Token::IDENT("ten".into()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
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
            Token::BANG,
            Token::MINUS,
            Token::DIVIDE,
            Token::MULTIPLY,
            Token::INTEGER("5".into()),
            Token::SEMICOLON,

            Token::INTEGER("5".into()),
            Token::LT,
            Token::INTEGER("10".into()),
            Token::GT,
            Token::INTEGER("5".into()),
            Token::SEMICOLON,

            Token::IF,
            Token::LPAREN,
            Token::INTEGER("5".into()),
            Token::LT,
            Token::INTEGER("10".into()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,

            Token::INTEGER("10".into()),
            Token::EQUAL,
            Token::INTEGER("10".into()),
            Token::SEMICOLON,
            
            Token::INTEGER("10".into()),
            Token::NOT_EQUAL,
            Token::INTEGER("9".into()),
            Token::SEMICOLON,
            Token::EOF,
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
            Token::LET,
            Token::IDENT("x".into()),
            Token::ASSIGN,
            Token::DOUBLE_QUOTE,
            Token::STRING("hello".into()),
            Token::DOUBLE_QUOTE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("y".into()),
            Token::ASSIGN,
            Token::DOUBLE_QUOTE,
            Token::STRING("world!".into()),
            Token::DOUBLE_QUOTE,
            Token::SEMICOLON,

            Token::LET,
            Token::IDENT("test".into()),
            Token::ASSIGN,
            Token::DOUBLE_QUOTE,
            Token::STRING("'abc123_!@#$%^&*(){};".into()),
            Token::DOUBLE_QUOTE,
            Token::SEMICOLON,
        ];

        for token in tokens {
            assert_eq!(token, l.next_token());
        }
    }
}
