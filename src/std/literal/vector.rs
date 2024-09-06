use std::rc::Rc;

use crate::{
    ast::{LiteralType, Token, Wrapper},
    errors::raw,
    std::{func, StdFunc},
};

impl StdFunc {
    pub fn load_literal_vector(&mut self) {
        self.load_push(None);
    }

    pub fn load_push(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "push".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 2 {
                        raw("expected two argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let mut v: Vec<LiteralType> = v.clone();
                                v.push(
                                    args.get(1)
                                        .unwrap()
                                        .as_ref()
                                        .unwrap_or(&LiteralType::Null)
                                        .clone(),
                                );
                                return LiteralType::Vec(v);
                            }
                            _ => {}
                        },
                        None => {}
                    };
                    LiteralType::Null
                }),
            }),
        );
    }
}
