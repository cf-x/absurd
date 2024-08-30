use crate::{
    ast::{LiteralType, Token, Wrapper},
    interpreter::env::Env,
    std::func,
    utils::errors::raw,
};
use colored::Colorize;
use std::{cell::RefCell, process::exit, rc::Rc};

pub struct StdCoreIo {
    env: Rc<RefCell<Env>>,
}

impl StdCoreIo {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        Self { env }
    }

    pub fn load(&mut self) {
        self.load_print(None);
        self.load_eprint(None);
        self.load_warn(None);
        self.load_panic(None);
        self.load_exit(None);
        self.load_read_num(None);
        self.load_read_str(None);
        self.load_read_char(None);
        self.load_read_bool(None);
    }

    /// print(text) - prints text to the terminal
    pub fn load_print(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "print".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected an argument");
                    }
                    println!("{}", args[0].clone().unwrap().to_string());
                    LiteralType::Void
                }),
            }),
        );
    }

    /// eprint(text) - prints error text to the terminal
    pub fn load_eprint(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "eprint".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected an argument");
                    }
                    println!("{}", args[0].clone().unwrap().to_string().red());
                    LiteralType::Void
                }),
            }),
        );
    }

    pub fn load_warn(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "warn".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected an argument");
                    }
                    println!("{}", args[0].clone().unwrap().to_string().yellow());
                    LiteralType::Void
                }),
            }),
        );
    }

    pub fn load_panic(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "panic".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected an argument");
                    }
                    println!("{}", args[0].clone().unwrap().to_string().red());
                    exit(0);
                }),
            }),
        );
    }

    pub fn load_exit(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "exit".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() > 0 {
                        if args[0].is_none() {
                            raw("expected an argument");
                        }
                        if let LiteralType::Number(val) = &args[0].clone().unwrap() {
                            exit(*val as i32);
                        }
                    }
                    exit(0);
                }),
            }),
        );
    }

    pub fn load_read_str(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "read_str".to_string(),
        };
        func(
            name.as_str(),
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[Option<LiteralType>]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    LiteralType::String(input)
                }),
            }),
        );
    }

    pub fn load_read_num(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "read_num".to_string(),
        };
        func(
            name.as_str(),
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[Option<LiteralType>]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<f32>().unwrap();
                    LiteralType::Number(input)
                }),
            }),
        );
    }

    pub fn load_read_bool(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "read_bool".to_string(),
        };
        func(
            name.as_str(),
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[Option<LiteralType>]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<bool>().unwrap();
                    LiteralType::Boolean(input)
                }),
            }),
        );
    }

    pub fn load_read_char(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "read_char".to_string(),
        };
        func(
            name.as_str(),
            0,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|_args: &[Option<LiteralType>]| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().chars().next().unwrap();
                    LiteralType::Char(input)
                }),
            }),
        );
    }
}
