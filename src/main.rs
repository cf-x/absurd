mod ast;
mod cli;
mod interpreter;
mod parser;
mod resolver;
mod std;
use cli::cli;
use manifest::Project;
mod bundler;
mod errors;
mod manifest;

pub const VERSION: &str = "0.21.0";

fn main() {
    let mut project = Project::new();
    project.load();
    cli(&mut project);
}

