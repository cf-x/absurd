use cli::cli;
mod analyzer;
mod ast;
mod bundler;
mod cli;
mod env;
mod errors;
mod expr;
mod interpreter;
mod lexer;
mod literals;
mod parser;
mod resolver;
mod std;

fn main() {
    cli();
}
