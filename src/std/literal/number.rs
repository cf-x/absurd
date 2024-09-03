use crate::{
    ast::{LiteralType, Token, Wrapper},
    errors::raw,
    std::{func, StdFunc},
};
use std::{process, rc::Rc};

impl StdFunc {
    pub fn load_literal_number(&mut self) {
        self.load_sqr(None);
        self.load_add(None);
        self.load_sub(None);
        self.load_mult(None);
        self.load_div(None);
        self.load_rem(None);
        self.load_sqrt(None);
        self.load_cbrt(None);
        self.load_pow(None);
        self.load_log(None);
        self.load_sin(None);
        self.load_asin(None);
        self.load_cos(None);
        self.load_acos(None);
        self.load_tan(None);
        self.load_atan(None);
        self.load_abs(None);
        self.load_floor(None);
        self.load_ceil(None);
        self.load_round(None);
        self.load_signum(None);
        self.load_hypot(None);
        self.load_exp(None);
        self.load_exp2(None);
        self.load_exp_m1(None);
        self.load_ln(None);
        self.load_max(None);
        self.load_min(None);
        self.load_avg(None);
        self.load_to_degrees(None);
        self.load_to_radians(None);
    }

    pub fn load_sqr(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "sqr".to_string(),
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
                    if let LiteralType::Number(n) = args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n * n)
                    } else {
                        raw("sqr() expects a number");
                        process::exit(1)
                    }
                }),
            }),
        );
    }

    pub fn load_add(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "add".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n + m),
                                _ => {
                                    raw("add() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("add() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_sub(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "sub".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n - m),
                                _ => {
                                    raw("sub() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("sub() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_mult(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "mult".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n * m),
                                _ => {
                                    raw("mult() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("mult() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_div(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "div".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n / m),
                                _ => {
                                    raw("div() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("div() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_rem(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "rem".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n % m),
                                _ => {
                                    raw("rem() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("rem() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_sqrt(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "sqrt".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.sqrt()),
                        _ => {
                            raw("sqrt() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_cbrt(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "cbrt".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.cbrt()),
                        _ => {
                            raw("cbrt() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_pow(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "pow".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n.powf(m)),
                                _ => {
                                    raw("pow() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("pow() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_log(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "log".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n.log(m)),
                                _ => {
                                    raw("log() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("log() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_sin(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "sin".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.sin()),
                        _ => {
                            raw("sin() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_asin(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "asin".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.asin()),
                        _ => {
                            raw("asin() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_cos(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "cos".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.cos()),
                        _ => {
                            raw("cos() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_acos(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "acos".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.acos()),
                        _ => {
                            raw("acos() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_tan(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "tan".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.tan()),
                        _ => {
                            raw("tan() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_atan(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "atan".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.atan()),
                        _ => {
                            raw("atan() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_abs(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "abs".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.abs()),
                        _ => {
                            raw("abs() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_floor(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "floor".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.floor()),
                        _ => {
                            raw("floor() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_ceil(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "ceil".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.ceil()),
                        _ => {
                            raw("ceil() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_round(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "round".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.round()),
                        _ => {
                            raw("round() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_signum(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "signum".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.signum()),
                        _ => {
                            raw("signum() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_hypot(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "hypot".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n.hypot(m)),
                                _ => {
                                    raw("hypot() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("hypot() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_exp(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "exp".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.exp()),
                        _ => {
                            raw("exp() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_exp2(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "exp2".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.exp2()),
                        _ => {
                            raw("exp2() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_exp_m1(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "exp_m1".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.exp_m1()),
                        _ => {
                            raw("exp_m1() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_ln(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "ln".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.ln()),
                        _ => {
                            raw("ln() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }
    pub fn load_max(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "max".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n.max(m)),
                                _ => {
                                    raw("max() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("max() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_min(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "min".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number(n.min(m)),
                                _ => {
                                    raw("min() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("min() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_avg(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "avg".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected 2 argument");
                    }
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => {
                            match args[1].clone().unwrap_or(LiteralType::Void) {
                                LiteralType::Number(m) => LiteralType::Number((n + m) / 2.0),
                                _ => {
                                    raw("avg() expects numbers");
                                    process::exit(1)
                                }
                            }
                        }
                        _ => {
                            raw("avg() expects numbers");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_to_degrees(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "to_degrees".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.to_degrees()),
                        _ => {
                            raw("to_degrees() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }

    pub fn load_to_radians(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "to_radians".to_string(),
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
                    match args[0].clone().unwrap_or(LiteralType::Void) {
                        LiteralType::Number(n) => LiteralType::Number(n.to_radians()),
                        _ => {
                            raw("to_radians() expects a number");
                            process::exit(1)
                        }
                    }
                }),
            }),
        );
    }
}
