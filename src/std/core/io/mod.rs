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
        self.load_eprint();
        self.load_warn();
        self.load_panic();
        self.load_exit();
        self.load_read_num();
        self.load_read_str();
        self.load_read_char();
        self.load_read_bool();
    }

    /// print(text) - prints text to the terminal
    pub fn load_print(&mut self) {
        func(
            "print",
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

    pub fn load_read_str(&mut self) {
        func(
            "read_str",
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[LiteralType]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    LiteralType::String(input)
                }),
            }),
        );
    }

    pub fn load_read_num(&mut self) {
        func(
            "read_num",
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[LiteralType]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<f32>().unwrap();
                    LiteralType::Number(input)
                }),
            }),
        );
    }

    pub fn load_read_bool(&mut self) {
        func(
            "read_bool",
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[LiteralType]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<bool>().unwrap();
                    LiteralType::Boolean(input)
                }),
            }),
        );
    }

    pub fn load_read_char(&mut self) {
        func(
            "read_char",
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[LiteralType]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().chars().next().unwrap();
                    LiteralType::Char(input)
                }),
            }),
        );
    }
}
