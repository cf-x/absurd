mod ast;
mod bundler;
mod errors;
mod interpreter;
mod parser;
mod resolver;
mod std;
use crate::bundler::interpreter;
use ::std::{fs::File, io::Read};
use abs_cli::CLI;
use errors::log;

// Constants values, initial values and language information
pub const VERSION: &str = "1.0.0";

/// `Config` struct is for managing configuration across the interpreter.
/// Struct has following fields:
///
/// * `test`: true if Absurd is running in `test mode`. `[stable]`
/// * `unsafe_mode`: true if unsafe features are enabled. `[planned]`
/// * `diagnostics`: true if diagnostic are enabled. `[planned]`
/// * `emit`: type of emit the interpeter should emit. `[planned]`
#[derive(Debug, Clone)]
pub struct Config {
    pub test: bool,
    pub unsafe_mode: bool,
    pub diagnostics: bool,
    pub emit: &'static str,
}
impl Config {
    pub fn new() -> Self {
        Self {
            test: false,
            unsafe_mode: false,
            diagnostics: false,
            emit: "default",
        }
    }
}

/// The `main` (entry) function is used for managing CLI and pre-processing.
fn main() {
    let mut config = Config::new();
    let mut program = CLI::new();
    program
        .name("Absurd")
        .version(VERSION)
        .description("Absurd Programming Language")
        .option("-t, --test", "enable testing mode")
        .option("-d, --diagnose", "run diagnostics for better debugging")
        .option(
            "-e, --emit",
            "type of output for the interpreter to emit ([default|tokens|ast|env])",
        )
        .arg("run", "run [file]", "interpret the file")
        .arg("lint", "lint [file]", "run the linter")
        .arg("format", "format [file]", "format the file")
        .arg("error", "error [code]", "get more info about the error");
    program.parse();

    // @todo handle flags with different args

    if program.get("--test").is_some() {
        config.test = true
    }

    if let Some(run) = program.get("run") {
        match run.get(0) {
            Some(r) => run_file(r, config),
            None => log("cli error: failed to get the target file"),
        }
    }
}

/// Function `run_file` reads and then interprets the target file
/// It takes two arguments and returns nothing, but emits the output.
fn run_file(path: &String, config: Config) {
    let mut file = match File::open(path.clone()) {
        Ok(s) => s,
        Err(_) => {
            log("cli error: failed to open the target:");
            eprintln!("{}", path);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            log("cli error: failed to read the target:");
            eprintln!("{}", path);
            return;
        }
    }
    interpreter(&contents, config.clone());
}
