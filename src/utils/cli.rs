use colored::Colorize;

use super::{errors::raw, manifest::Project};
use crate::{utils::bundler::interpreter_raw, VERSION};
use std::{
    env,
    fs::File,
    io::Read,
    process::{self, Command, Stdio},
};

struct Args {
    file: String,
}

fn print_help() {
    println!("\n");
    println!(
        "{} {} {} {}",
        "usage:".yellow(),
        "aperture".red(),
        "<file>".blue(),
        "[OPTIONS]".green()
    );
    println!();
    println!("{}", "Options:".yellow());
    println!(
        "  {}           {}",
        "--help, -h".blue(),
        "print this message"
    );
    println!(
        "  {}        {}",
        "--version, -v".blue(),
        "print current version"
    );
    println!(
        "  {}   {}",
        "--side-effects, -s".blue(),
        "disable side-effects"
    );
    println!();
    println!("{}", "Arguments:".yellow());
    println!(
        "  {}               {}",
        "<file>".blue(),
        "file to interpret"
    );
    println!(
        "  {}               {}",
        "update".blue(),
        "update to latest version"
    );
    println!("");
    println!("{} \n", "happy coding ッ".green())
}

fn print_version() {
    println!("\tversion {}", VERSION.red());
}

fn parse_args(project: &mut Project) -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        raw("missing required argument <file>");
    }

    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                get_wall();
                print_version();
                process::exit(0);
            }
            "update" => {
                update();
                process::exit(0);
            }
            "--side-effects" | "-s" => {
                project.side_effects = false;
            }
            _ => {}
        }
    }

    Args {
        file: args[1].clone(),
    }
}

pub fn cli(project: &mut Project) {
    let args = parse_args(project);
    run(args.file, project.clone())
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

fn update() {
    let curl_output = Command::new("curl")
        .arg("-sSL")
        .arg("https://static.ykk2b.xyz/install.sh")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute curl command");

    if !curl_output.status.success() {
        eprintln!("curl command failed with status: {}", curl_output.status);
        std::process::exit(1);
    }

    let mut bash_output = Command::new("bash")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute bash command");

    if let Some(bash_stdin) = bash_output.stdin.as_mut() {
        use std::io::Write;
        bash_stdin
            .write_all(&curl_output.stdout)
            .expect("failed to write to bash stdin");
    }

    let bash_status = bash_output.wait().expect("failed to wait on bash command");

    if !bash_status.success() {
        eprintln!("bash command failed with status: {}", bash_status);
        std::process::exit(1);
    }
}

fn get_wall() {
    let a = r#"
        _______  _____  _______  ______ _______ _     _  ______ _______
        |_____| |_____] |______ |_____/    |    |     | |_____/ |______
        |     | |       |______ |    \_    |    |_____| |    \_ |______
    "#;

    println!("{}", a.red());
}
