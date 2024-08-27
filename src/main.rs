use utils::cli::cli;
use utils::manifest::Project;
// mod analyzer;
mod ast;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod std;
mod utils;

pub const VERSION: &str = "0.13.0";

fn main() {
    let mut project = Project::new();
    project.load();
    cli(&mut project);
}
