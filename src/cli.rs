use abs_cli::CLI;
use coloredpp::Colorize;
use std::{
    fs::File,
    io::{stdin, Read, Write},
    process::{exit, Command, Stdio},
};

use crate::{bundler::interpreter_raw, errors::raw, manifest::Project, VERSION};
pub fn cli_new(project: &mut Project) {
    let mut program = CLI::new();
    program
        .name("Absurd")
        .version(VERSION)
        .description("The Absurd Programming Language")
        .option("-s, --side-effects", "disable side-effects")
        .option("-l, --log", "enable logging mode")
        .option("-t, --test", "enable testing mode")
        .arg("run", "run [file]", "interpret the file")
        .arg("update", "update", "update to the latest version")
        .arg("ci", "ci", "enter source from the CLI")
        .arg(
            "add",
            "add [repo]/[name] <new_name>",
            "add a new package to the project",
        )
        .arg(
            "remove",
            "remove [name]",
            "remove the package from the project",
        );
    program.parse();

    if program.get("--test").is_some() {
        project.test = true
    }
    if program.get("--side-effects").is_some() {
        project.side_effects = false
    }
    if program.get("--log").is_some() {
        project.log = true
    }
    if program.get("update").is_some() {
        update();
        exit(0);
    }

    let run = program.get("run");
    if run.is_some() {
        run_file(
            run.unwrap().get(0).expect("expected a file").clone(),
            project.clone(),
        );
        exit(1);
    }

    let add = program.get("add");
    if add.is_some() {
        add_mod(add.unwrap().get(0), add.unwrap().get(1));
        exit(1);
    }

    let remove = program.get("add");
    if remove.is_some() {
        remove_mod(remove.unwrap().get(0));
        exit(1);
    }

    if program.get("ci").is_some() {
        println!("Enter your code (end with Ctrl+D):");
        let mut code_input = String::new();
        stdin()
            .read_to_string(&mut code_input)
            .expect("Failed to read input");
        println!("");
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
