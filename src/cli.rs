use crate::{bundler::interpreter_raw, manifest::Project};
use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file to run
    file: String,
}

pub fn cli(project: Project) {
    let args = Args::parse();
    run(args.file, project)
}

fn run(file: String, project: Project) {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    interpreter_raw(&contents, project);
}
