use super::{errors::raw, manifest::Project};
use crate::{utils::bundler::interpreter_raw, VERSION};
use core::panic;
use std::{env, fs::File, io::Read, process};

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

fn run(f: String, project: Project) {
    let mut file = match File::open(f.clone()) {
        Ok(s) => s,
        Err(_) => {
            raw(format!("failed to open file '{f}'").as_str());
            process::exit(1);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            raw(format!("failed to read file '{f}'").as_str());
            process::exit(1);
        }
    }

    interpreter_raw(&contents, project);
}
