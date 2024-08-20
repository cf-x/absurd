use crate::bundler::interpreter_raw;
use std::{env, fs::File, io::Read};

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no command provided");
        return;
    }
    let command = args[1].clone();
    let command = command.as_str();
    match command {
        "run" => run(args),
        "help" => help(),
        _ => {
            println!("invalid command");
        }
    }
}

fn help() {
    println!("Aperture v0.7.1");
    println!("USAGE:");
    println!("run <file> - run a file");
    println!("help - show this message");
}

fn run(args: Vec<String>) {
    if args.len() < 3 {
        println!("no file provided");
        return;
    }
    let file = args[2].clone();
    let file = file.as_str();
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    interpreter_raw(&contents);
}
