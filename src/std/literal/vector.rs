use std::rc::Rc;

use crate::{
    ast::{LiteralType, Token, Wrapper},
    errors::raw,
    std::{func, StdFunc},
};

impl StdFunc {
    pub fn load_literal_vector(&mut self) {
        self.load_push(None);
        self.load_to_string(None);
        self.load_join(None);
        self.load_first(None);
        self.load_last(None);
        self.load_pop(None);
        self.load_reverse(None);
        self.load_for_each(None);
        self.load_connect(None);
        self.load_has(None);
        self.load_key(None);
        self.load_get(None);
    }

    /// push(vector, item);
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

    /// get(vector, index);
    pub fn load_get(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "get".to_string(),
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
                                let v: Vec<LiteralType> = v.clone();
                                let i = if let Some(Some(LiteralType::Number(n))) = args.get(1) {
                                    *n as usize
                                } else {
                                    0
                                };
                                return v.get(i).unwrap_or(&LiteralType::Null).clone();
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

    /// key(vector, item)
    pub fn load_key(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "key".to_string(),
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
                                let v: Vec<LiteralType> = v.clone();
                                let mut key = -1;
                                v.iter().for_each(|p| {
                                    if p != args
                                        .get(1)
                                        .unwrap()
                                        .as_ref()
                                        .unwrap_or(&LiteralType::Null)
                                    {
                                        key += 1;
                                    }
                                });
                                return LiteralType::Number(key as f32);
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

    /// has(vector, item)
    pub fn load_has(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "has".to_string(),
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
                                let v: Vec<LiteralType> = v.clone();
                                let mut bool = false;
                                v.iter().for_each(|p| {
                                    if p == args
                                        .get(1)
                                        .unwrap()
                                        .as_ref()
                                        .unwrap_or(&LiteralType::Null)
                                    {
                                        bool = true;
                                    }
                                });

                                return LiteralType::Boolean(bool);
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

    /// connect(array, array);
    pub fn load_connect(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "connect".to_string(),
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
                                let v2 = if let Some(Some(LiteralType::Vec(vc))) = args.get(1) {
                                    vc.clone()
                                } else {
                                    vec![]
                                };
                                v2.iter().for_each(|c| v.push(c.clone()));
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

    /// for_each(vector, |item| void)
    pub fn load_for_each(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "for_each".to_string(),
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
                                let v: Vec<LiteralType> = v.clone();
                                if let Some(Some(LiteralType::DeclrFunc(f))) = args.get(1) {
                                    v.iter().for_each(|c| {
                                        (*f.func).call(vec![Some(c.clone())]);
                                    });
                                };
                                return LiteralType::Void;
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

    pub fn load_reverse(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "reverse".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected one argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let mut v: Vec<LiteralType> = v.clone();
                                v.reverse();
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

    pub fn load_pop(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "pop".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected one argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let mut v: Vec<LiteralType> = v.clone();
                                v.pop();
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

    pub fn load_last(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "last".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected one argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let mut v: Vec<LiteralType> = v.clone();
                                return v.pop().unwrap_or(LiteralType::Null);
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

    pub fn load_first(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "first".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected one argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let mut v: Vec<LiteralType> = v.clone();
                                v.reverse();
                                return v.pop().unwrap_or(LiteralType::Null);
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

    /// join(vec, string)
    pub fn load_join(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "join".to_string(),
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
                                let v: Vec<LiteralType> = v.clone();
                                let sep = if let Some(Some(LiteralType::String(s))) = args.get(1) {
                                    s.clone()
                                } else {
                                    "".to_string()
                                };

                                let mut s = String::new();
                                for (_, v) in v.iter().enumerate() {
                                    s.push_str(&v.to_string());
                                    sep.chars().for_each(|f| s.push(f))
                                }

                                return LiteralType::String(s);
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

    /// to_string(vec)
    pub fn load_to_string(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "to_string".to_string(),
        };
        func(
            name.as_str(),
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[Option<LiteralType>]| {
                    if args.len() != 1 {
                        raw("expected one argument");
                    }
                    match args.get(0) {
                        Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
                            LiteralType::Vec(v) => {
                                let v: Vec<LiteralType> = v.clone();

                                let mut s = String::new();
                                for (_, v) in v.iter().enumerate() {
                                    s.push_str(&v.to_string());
                                }

                                return LiteralType::String(s);
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

    //// map(vector, |item| void)
    // pub fn load_map(&mut self, name: Option<Token>) {
    //     let name = match name {
    //         Some(n) => n.lexeme.clone(),
    //         None => "map".to_string(),
    //     };
    //     func(
    //         name.as_str(),
    //         1,
    //         &mut self.env,
    //         Rc::new(Wrapper {
    //             0: Box::new(|args: &[Option<LiteralType>]| {
    //                 if args.len() != 2 {
    //                     raw("expected two argument");
    //                 }
    //                 match args.get(0) {
    //                     Some(v) => match v.as_ref().unwrap_or(&LiteralType::Null) {
    //                         LiteralType::Vec(v) => {
    //                             let v: Vec<LiteralType> = v.clone();
    //                             if let Some(Some(LiteralType::DeclrFunc(f))) = args.get(1) {
    //                                 v.iter().for_each(|c| {
    //                                     return (*f.func).call(vec![Some(c.clone())]);
    //                                 });
    //                             } else {
    //                                 return LiteralType::Null
    //                             };
    //                             return LiteralType::Void;
    //                         }
    //                         _ => {}
    //                     },
    //                     None => {}
    //                 };
    //                 LiteralType::Null
    //             }),
    //         }),
    //     );
    // }
}
