use crate::{lexer::Lexer, parser::Parser, evaluator};
use std::io;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = ">> ";
    loop {
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout())?;
        for line in io::stdin().lines() {
            if let Ok(line) = line {
                let lex = Lexer::new(line);
                let mut parser = Parser::new(lex);

                let program = parser.parse_program();

                match program {
                    Ok(program) => {
                        let evaluated = evaluator::eval(&program);
                        println!("{}", evaluated.inspect());
                    }
                    Err(_) => {
                        for e in parser.errors() {
                            println!("{}", e);
                        }
                    }
                }
                break;
            }
        }
    }
}
