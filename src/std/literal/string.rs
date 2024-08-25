use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{LiteralType, Token, Wrapper},
    interpreter::env::Env,
    std::func,
};

pub struct StdLiteralString {
    env: Rc<RefCell<Env>>,
}

impl StdLiteralString {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        Self { env }
    }

    pub fn load(&mut self) {
        self.load_string(None);
        // self.load_chars(None);
        self.load_chars_count(None);
        self.load_contains(None);
        self.load_find(None);
        self.load_ends_with(None);
        self.load_starts_with(None);
        self.load_is_empty(None);
        self.load_len(None);
        // self.load_lines(None);
        self.load_to_lowercase(None);
        self.load_to_uppercase(None);
        // self.load_parse(None);
        self.load_replace(None);
        // self.load_split(None);
        // self.load_split_once(None);
        // self.load_split_whitespace(None);
        self.load_trim(None);
        self.load_trim_start(None);
        self.load_trim_end(None);
    }

    pub fn load_string(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "string".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| LiteralType::String(args[0].to_string())),
            }),
        );
    }

    // pub fn load_chars(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "chars".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         1,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match &args[0] {
    //                 LiteralType::String(s) => {
    //                     LiteralType::Array(s.chars().map(LiteralType::Char).collect())
    //                 }
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    pub fn load_chars_count(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "chars_count".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(s) => LiteralType::Number(s.chars().count() as f32),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_contains(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "contains".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
                    (LiteralType::String(a), LiteralType::String(b)) => {
                        LiteralType::Boolean(a.contains(b))
                    }
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_find(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "find".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
                    (LiteralType::String(a), LiteralType::String(b)) => {
                        if let Some(index) = a.find(b) {
                            LiteralType::Number(index as f32)
                        } else {
                            LiteralType::Null
                        }
                    }
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_ends_with(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "ends_with".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
                    (LiteralType::String(a), LiteralType::String(b)) => {
                        LiteralType::Boolean(a.ends_with(b))
                    }
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_starts_with(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "starts_with".to_string(),
        };
        func(
            name.as_str(),
            2,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
                    (LiteralType::String(a), LiteralType::String(b)) => {
                        LiteralType::Boolean(a.starts_with(b))
                    }
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_is_empty(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "is_empty".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::Boolean(a.is_empty()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_len(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "len".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::Number(a.len() as f32),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    // pub fn load_lines(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "lines".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         1,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match &args[0] {
    //                 LiteralType::String(a) => LiteralType::Array(
    //                     a.lines()
    //                         .map(|line| LiteralType::String(line.to_string()))
    //                         .collect(),
    //                 ),
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    pub fn load_to_lowercase(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "to_lowercase".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::String(a.to_lowercase()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_to_uppercase(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "to_uppercase".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::String(a.to_uppercase()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    // pub fn load_parse(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "parse".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         1,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match &args[0] {
    //                 LiteralType::String(a) => {
    //                     if let Ok(parsed) = a.parse::<f64>() {
    //                         LiteralType::Number(parsed)
    //                     } else {
    //                         LiteralType::Null
    //                     }
    //                 }
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    pub fn load_replace(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "replace".to_string(),
        };
        func(
            name.as_str(),
            3,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(
                    |args: &[LiteralType]| match (&args[0], &args[1], &args[2]) {
                        (
                            LiteralType::String(a),
                            LiteralType::String(b),
                            LiteralType::String(c),
                        ) => LiteralType::String(a.replace(b, c)),
                        _ => LiteralType::Null,
                    },
                ),
            }),
        );
    }

    // pub fn load_split(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "split".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         2,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
    //                 (LiteralType::String(a), LiteralType::String(b)) => LiteralType::Array(
    //                     a.split(b)
    //                         .map(|s| LiteralType::String(s.to_string()))
    //                         .collect(),
    //                 ),
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    // pub fn load_split_once(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "split_once".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         2,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match (&args[0], &args[1]) {
    //                 (LiteralType::String(a), LiteralType::String(b)) => {
    //                     if let Some((first, second)) = a.split_once(b) {
    //                         LiteralType::Array(vec![
    //                             LiteralType::String(first.to_string()),
    //                             LiteralType::String(second.to_string()),
    //                         ])
    //                     } else {
    //                         LiteralType::Null
    //                     }
    //                 }
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    // pub fn load_split_whitespace(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "split_whitespace".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         1,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[LiteralType]| match &args[0] {
    //                 LiteralType::String(a) => {
    //                     let split_result: Vec<LiteralType> = a
    //                         .split_whitespace()
    //                         .map(|s| LiteralType::String(s.to_string()))
    //                         .collect();
    //                     LiteralType::Array(split_result)
    //                 }
    //                 _ => LiteralType::Null,
    //             }),
    //         }),
    //     );
    // }

    pub fn load_trim(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "trim".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::String(a.trim().to_string()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_trim_start(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "trim_start".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::String(a.trim_start().to_string()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }

    pub fn load_trim_end(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "trim_end".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| match &args[0] {
                    LiteralType::String(a) => LiteralType::String(a.trim_end().to_string()),
                    _ => LiteralType::Null,
                }),
            }),
        );
    }
}
