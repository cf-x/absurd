use crate::{
    ast::{LiteralType, Wrapper},
    env::Env,
    std::func,
};
use colored::Colorize;
use std::{process::exit, rc::Rc};

pub struct StdCoreIo {
    env: Env,
}

impl StdCoreIo {
    pub fn new(env: Env) -> Self {
        Self { env }
    }

    pub fn load(&mut self) {
        self.load_print();
        self.load_println();
    }

    /// print(text) - prints text to the terminal
    pub fn load_print(&mut self) {
        func(
            "print",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    print!("{}", args[0].to_string());
                    LiteralType::Void
                }),
            }),
        );
    }

    /// println(text) - prints text to the terminal with a newline
    pub fn load_println(&mut self) {
        func(
            "println",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    println!("{}", args[0].to_string());
                    LiteralType::Void
                }),
            }),
        );
    }

    /// eprint(text) - prints error text to the terminal
    pub fn load_eprint(&mut self) {
        func(
            "eprint",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    print!("{}", args[0].to_string().red());
                    LiteralType::Void
                }),
            }),
        );
    }

    /// eprintln(text) - prints error text to the terminal with a newline
    pub fn load_eprintln(&mut self) {
        func(
            "eprintln",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    println!("{}", args[0].to_string().red());
                    LiteralType::Void
                }),
            }),
        );
    }

    pub fn load_warn(&mut self) {
        func(
            "warn",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    print!("{}", args[0].to_string().yellow());
                    LiteralType::Void
                }),
            }),
        );
    }

    pub fn load_warnln(&mut self) {
        func(
            "warnln",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    println!("{}", args[0].to_string().yellow());
                    LiteralType::Void
                }),
            }),
        );
    }

    pub fn load_panic(&mut self) {
        func(
            "panic",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    println!("{}", args[0].to_string().red());
                    exit(0);
                }),
            }),
        );
    }

    pub fn load_exit(&mut self) {
        func(
            "exit",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    if args.len() > 0 {
                        if let LiteralType::Number(val) = &args[0] {
                            exit(*val as i32);
                        }
                    }
                    exit(0);
                }),
            }),
        );
    }
}
