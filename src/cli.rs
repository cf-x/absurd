use std::{
    env,
    fs::File,
    io::{stdin, Read, Write},
    process::{exit, Command, Stdio},
};

use coloredpp::Colorize;

use crate::{bundler::interpreter_raw, errors::raw, manifest::Project, VERSION};

struct Args {
    file: Option<String>,
    code_input: Option<String>,
}

fn print_help() {
    println!("\n");
    println!(
        "{} {} {} {}",
        "usage:".bright_yellow(),
        "absurd".bright_red(),
        "<file>".bright_blue(),
        "[OPTIONS]".bright_green()
    );
    println!();
    println!("{}", "Options:".bright_yellow());
    println!(
        "  {}           {}",
        "--help, -h".bright_blue(),
        "print this message"
    );
    println!(
        "  {}        {}",
        "--version, -v".bright_blue(),
        "print current version"
    );
    println!(
        "  {}   {}",
        "--side-effects, -s".bright_blue(),
        "disable side-effects"
    );
    println!(
        "  {}            {}",
        "--log, -l".bright_blue(),
        "enable logging mode"
    );
    println!(
        "  {}           {}",
        "--test, -t".bright_blue(),
        "enable testing mode"
    );
    println!();
    println!("{}", "Arguments:".bright_yellow());
    println!(
        "  {}               {}",
        "<file>".bright_blue(),
        "file to interpret"
    );
    println!(
        "  {}               {}",
        "update".bright_blue(),
        "update to latest version"
    );
    println!(
        "  {}                   {}",
        "ci".bright_blue(),
        "enter code directly in the CLI"
    );
    println!("");
    println!("{} \n", "happy coding ッ".bright_green())
}

fn parse_args(project: &mut Project) -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        raw("missing required argument <file>");
    }
    for (i, arg) in &args.iter().enumerate().collect::<Vec<(usize, &String)>>() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                exit(0);
            }
            "--version" | "-v" => {
                print_version();
                exit(0);
            }
            "update" => {
                update();
                exit(0);
            }
            "add" => {
                add_mod(args.clone().get(i + 1), args.clone().get(i + 2));
                exit(1);
            }
            "remove" => {
                remove_mod(args.clone().get(i + 1));
                exit(1);
            }
            "--side-effects" | "-s" => project.side_effects = false,
            "--log" | "-l" => project.log = true,
            "--test" | "-t" => project.test = true,
            "ci" => {
                println!("Enter your code (end with Ctrl+D):");
                let mut code_input = String::new();
                stdin()
                    .read_to_string(&mut code_input)
                    .expect("Failed to read input");
                println!("");
                return Args {
                    file: None,
                    code_input: Some(code_input),
                };
            }
            _ => {}
        }
    }

    Args {
        file: Some(args[1].clone()),
        code_input: None,
    }
}

pub fn cli(project: &mut Project) {
    let args = parse_args(project);
    if let Some(file) = args.file {
        run_file(file, project.clone());
    } else if let Some(code_input) = args.code_input {
        run_code(code_input, project.clone());
    }
}

fn run_file(f: String, project: Project) {
    let mut file = match File::open(f.clone()) {
        Ok(s) => s,
        Err(_) => {
            raw(format!("failed to open file '{f}'").as_str());
            exit(1);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            raw(format!("failed to read file '{f}'").as_str());
            exit(1);
        }
    }

    interpreter_raw(&contents, project.clone(), project.log);
}

fn run_code(code: String, project: Project) {
    interpreter_raw(&code, project.clone(), project.log);
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
        exit(1);
    }

    let mut bash_output = Command::new("bash")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute bash command");

    if let Some(bash_stdin) = bash_output.stdin.as_mut() {
        bash_stdin
            .write_all(&curl_output.stdout)
            .expect("failed to write to bash stdin");
    }

    let bash_status = bash_output.wait().expect("failed to wait on bash command");

    if !bash_status.success() {
        eprintln!("bash command failed with status: {}", bash_status);
        exit(1);
    }
}

fn add_mod(first: Option<&String>, second: Option<&String>) {
    let name = if let Some(n) = first {
        n.clone()
    } else {
        raw("expected a module name");
        exit(1);
    };

    if let Some(alias) = second {
        println!("adding: {} as {}", name, alias);
        return;
    }

    let dir_name = if second.is_some() {
        second.unwrap()
    } else {
        name.split('/').last().unwrap_or("mods")
    };
    println!("{}", format!("cloning module {}...", name).yellow());
    let output = match Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(format!("https://github.com/{}.git", name))
        .arg(format!("./mods/{}", dir_name))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(c) => c,
        Err(e) => {
            raw(format!("module error: {}", e).as_str());
            exit(1);
        }
    };
    println!(
        "{}",
        format!("module {} successfully cloned", dir_name).green()
    );
    /* @todo:
        - add metadata to the project.toml
        - add package to the environment (lib.abs)
    */
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        raw(format!("git error: {}", stderr).as_str());
        exit(1);
    }

    println!("{}", "module successfully installed".green());
}

fn remove_mod(first: Option<&String>) {
    let name = if let Some(n) = first {
        n.clone()
    } else {
        raw("expected a module name");
        exit(1);
    };

    let output = match Command::new("rm")
        .arg("-rf")
        .arg(format!("./mods/{}", name))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(c) => c,
        Err(e) => {
            raw(format!("module error: {}", e).as_str());
            exit(1);
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        raw(format!("rm error: {}", stderr).as_str());
        exit(1);
    }

    println!(
        "{}",
        format!("module {} successfully removed", name).green()
    );
}

fn print_version() {
    let abs = VERSION.fg_hex_gradient("#6800ff", "#ff00aa").bold();
    println!("\n    {}: {}\n", "version".cyan(), abs);
}
