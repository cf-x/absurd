use cli::cli;
use manifest::Project;
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
mod manifest;
mod parser;
mod resolver;
mod std;

fn main() {
    let mut project = Project::new();
    project.load();
    
    cli(project);
}
