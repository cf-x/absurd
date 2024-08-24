use crate::{utils::bundler::interpreter_raw, VERSION};
use std::{env, fs::File, io::Read, process};
use super::manifest::Project;

struct Args {
    file: String,
}

fn print_help() {
    println!("Usage: aperture [OPTIONS] <file>");
    println!();
    println!("Options:");
    println!("  --help       Print help information");
    println!("  --version    Print version information");
    println!();
    println!("Arguments:");
    println!("  <file>       The file to run");
}

fn print_version() {
    println!("Aperture version {}", VERSION);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("@error: Missing required argument <file>");
    }

    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                print_version();
                process::exit(0);
            }
            _ => {}
        }
    }

    Args {
        file: args[1].clone(),
    }
}

pub fn cli(project: Project) {
    let args = parse_args();
    run(args.file, project)
}

fn run(file: String, project: Project) {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    interpreter_raw(&contents, project);
}
