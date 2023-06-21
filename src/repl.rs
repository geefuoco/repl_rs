use crate::{evaluator, lexer::Lexer, object::Environment, parser::Parser};
use std::io;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = ">> ";
    let mut env = Environment::new();
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
                        let evaluated = evaluator::eval_program(&program, &mut env);
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
