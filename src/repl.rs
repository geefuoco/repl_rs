use crate::{lexer::Lexer, parser::Parser};
use std::io;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = ">> ";
    loop {
        println!("{}", prompt);
        for line in io::stdin().lines() {
            if let Ok(line) = line {
                let lex = Lexer::new(line);
                let mut parser = Parser::new(lex);

                let program = parser.parse_program();

                match program {
                    Ok(program) => {
                        println!("{}", program)
                    }
                    Err(_) => {
                        for e in parser.errors() {
                            println!("{}", e);
                        }
                    }
                }
            }
        }
    }
}
