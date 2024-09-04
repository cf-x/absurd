// mod analyzer;
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

pub const VERSION: &str = "0.16.0";

fn main() {
    // load manifest
    let mut project = Project::new();
    project.load();
    // load cli
    cli(&mut project);
}

// @todo handle `release`, `rc` and `dev` branches
