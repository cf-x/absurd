use crate::{
    ast::Token,
    std::core::{io::StdCoreIo, load_core},
    utils::errors::raw,
};

use super::Interpreter;

impl Interpreter {
    pub fn load_std(&self, src: String, names: Vec<(Token, Option<Token>)>, all: bool) {
        let parts: Vec<&str> = src.split("::").collect();

        if parts[0] != "std" {
            raw(format!("standard library '{src}' doesn't exists").as_str());
        }

        if parts.len() >= 2 {
            match parts[1] {
                "core" => {
                    if parts.len() >= 3 {
                        match parts[2] {
                            "io" => {
                                let mut std_core_io = StdCoreIo::new(self.env.clone());
                                if all {
                                    std_core_io.load();
                                }
                                for (name, name2) in names {
                                    match name.lexeme.as_str() {
                                        "print" => {
                                            std_core_io.load_print(name2);
                                        }
                                        "eprint" => {
                                            std_core_io.load_eprint(name2);
                                        }
                                        "warn" => {
                                            std_core_io.load_warn(name2);
                                        }
                                        "panic" => {
                                            std_core_io.load_panic(name2);
                                        }
                                        "exit" => {
                                            std_core_io.load_exit(name2);
                                        }
                                        "read_num" => {
                                            std_core_io.load_read_num(name2);
                                        }
                                        "read_str" => {
                                            std_core_io.load_read_str(name2);
                                        }
                                        "read_char" => {
                                            std_core_io.load_read_char(name2);
                                        }
                                        "read_bool" => {
                                            std_core_io.load_read_bool(name2);
                                        }
                                        _ => {
                                            raw(format!(
                                                "std module 'core::io::{}' doesn't exists",
                                                name.lexeme
                                            )
                                            .as_str());
                                        }
                                    }
                                }
                            }
                            _ => {
                                raw(format!("std module 'core::{}' doesn't exists", parts[2])
                                    .as_str());
                            }
                        }
                    } else {
                        load_core(self.env.clone())
                    }
                }
                _ => {
                    raw(format!("std module '{}' doesn't exists", parts[1]).as_str());
                }
            }
        }
    }
}
