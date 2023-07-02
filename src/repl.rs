use crate::object::Object;
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
                match line.as_str() {
                    "quit" | "exit" => std::process::exit(0),
                    _ => {}
                }
                let lex = Lexer::new(line);
                let mut parser = Parser::new(lex);

                let mut program = parser.parse_program();

                match &mut program {
                    Ok(program) => {
                        let evaluated = evaluator::eval_program(program, &mut env);
                        if let Some(evaluated) = evaluated {
                            println!("{}", evaluated.inspect());
                        }
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
