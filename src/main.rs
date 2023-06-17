mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;

fn main() {
    repl::start();
}
