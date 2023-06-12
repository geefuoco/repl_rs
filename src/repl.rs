use std::io;
use crate::lexer::{Lexer, Token};


pub fn start() -> Result<(), Box<dyn std::error::Error>>{
    let PROMPT = ">> ";
    loop {
        println!("{}", PROMPT);
        for line in io::stdin().lines() {
            if let Ok(line) = line {
                let mut lex = Lexer::new(line); 
                let mut tok = lex.next_token();

                loop {
                    if tok == Token::EOF{
                        break;
                    }
                    println!("{}", tok);
                    tok = lex.next_token();
                }
            }
        }
    }
    Ok(())
}
