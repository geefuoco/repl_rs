use crate::lexer::{Lexer, Token};
use std::io;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = ">> ";
    loop {
        println!("{}", prompt);
        for line in io::stdin().lines() {
            if let Ok(line) = line {
                let mut lex = Lexer::new(line);
                let mut tok = lex.next_token();

                loop {
                    if tok == Token::Eof {
                        break;
                    }
                    println!("{}", tok);
                    tok = lex.next_token();
                }
            }
        }
    }
}
