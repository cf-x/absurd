// handles Absurd manifesto file
use ::std::{fs::File, io::Read};
use std::process::exit;
use toml::{from_str, Value};

use super::errors::raw;

#[derive(Debug, Clone)]
pub struct Project {
    // # config
    pub snippet: i8,
    pub side_effects: bool,
    pub disable_std: bool,
    pub load_std: bool,
    pub disable_analyzer: bool,
    pub log: bool,
    pub test: bool,
}

impl Project {
    pub fn new() -> Self {
        Self {
            // # config
            snippet: 1,
            side_effects: true,
            disable_std: false,
            load_std: true,
            disable_analyzer: true,
            log: false,
            test: false,
        }
    }

    pub fn load(&mut self) {
        match File::open("project.toml") {
            Ok(f) => {
                let mut f = f;
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(_) => {
                        raw(format!("failed to read file 'project.toml'").as_str());
                        exit(1);
                    }
                }
                let parsed: Value = from_str(&contents).expect("failed to parse manifest");
                match parsed.as_table() {
                    Some(v) => {
                        if v.contains_key("config") {
                            let table = v.get("config").unwrap();
                            if table.get("snippet").is_some() {
                                self.snippet = self.get_int(table, "snippet");
                            }
                            if table.get("side_effects").is_some() {
                                self.side_effects = self.get_bool(table, "side_effects");
                            }
                            if table.get("disable_std").is_some() {
                                self.disable_std = self.get_bool(table, "disable_std");
                            }
                            if table.get("load_std").is_some() {
                                self.load_std = self.get_bool(table, "load_std");
                            }
                            if table.get("disable_analyzer").is_some() {
                                self.disable_analyzer = self.get_bool(table, "disable_analyzer");
                            }
                        }
                    }
                    None => {
                        raw("failed to parse manifest");
                        exit(1);
                    }
                }
            }
            Err(_) => {
                // raw(format!("failed to open file 'project.toml'").as_str());
                // exit(1);
            }
        };
    }

    fn get_bool(&self, table: &Value, name: &str) -> bool {
        table.get(name).unwrap().as_bool().unwrap()
    }

    fn get_int(&self, table: &Value, name: &str) -> i8 {
        table.get(name).unwrap().as_integer().unwrap() as i8
    }
}

// @todo enlarge the manifesto for analyzer, linter and interpreter in general
