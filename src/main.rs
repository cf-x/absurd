mod ast;
mod cli;
mod interpreter;
mod parser;
mod resolver;
mod std;
use cli::cli_new;
use manifest::Project;
mod bundler;
mod errors;
mod manifest;

pub const VERSION: &str = "0.25.0";

fn main() {
    let mut project = Project::new();
    project.load();
    cli_new(&mut project);
}
