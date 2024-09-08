use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub mod core;
pub mod literal;

use crate::{
    ast::{DeclrFuncType, FuncValType, LiteralType, Token},
    errors::raw,
    interpreter::{
        env::{Env, FuncKind},
        Interpreter,
    },
};

pub fn func(name: &str, arity: usize, env: &mut Rc<RefCell<Env>>, func: Rc<dyn FuncValType>) {
    let params = vec![];
    env.borrow().define_pub_func(
        name.to_string(),
        LiteralType::DeclrFunc(DeclrFuncType {
            name: name.to_string(),
            arity,
            func,
        }),
        FuncKind {
            params,
            is_async: false,
            is_pub: false,
        },
    )
}

#[derive(Clone)]
pub struct StdFunc {
    env: Rc<RefCell<Env>>,
    is_test: bool,
}

impl StdFunc {
    pub fn new(env: Rc<RefCell<Env>>, is_test: bool) -> Self {
        Self { env, is_test }
    }
}

impl Interpreter {
    pub fn std_map(
        &mut self,
    ) -> HashMap<&str, Vec<(&str, HashMap<&str, Box<dyn FnMut(&Option<Token>) + '_>>)>> {
        let std = StdFunc::new(Rc::clone(&self.env), self.project.test);
        HashMap::from([
            (
                "core",
                vec![
                    (
                        "io",
                        HashMap::from([
                            (
                                "print",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_print(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "eprint",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_print(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "warn",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_warn(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "panic",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_panic(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "exit",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_exit(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "read_num",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_read_num(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "read_str",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_read_str(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "read_char",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_read_char(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "read_bool",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_read_bool(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                        ]),
                    ),
                    (
                        "test",
                        HashMap::from([(
                            "assert",
                            Box::new({
                                let mut std = std.clone();
                                move |name2: &Option<Token>| {
                                    std.load_assert(name2.clone());
                                }
                            }) as Box<dyn FnMut(&Option<Token>)>,
                        )]),
                    ),
                ],
            ),
            (
                "literal",
                vec![
                    (
                        "number",
                        HashMap::from([
                            (
                                "sqr",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_sqr(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "add",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_add(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "sub",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_sub(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "mult",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_mult(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "div",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_div(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "rem",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_rem(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "sqrt",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_sqrt(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "cbrt",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_cbrt(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "pow",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_pow(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "log",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_log(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "sin",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_sin(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "asin",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_asin(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "cos",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_cos(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "acos",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_acos(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "tan",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_tan(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "atan",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_atan(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "abs",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_abs(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "floor",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_floor(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "ceil",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_ceil(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "round",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_round(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "signum",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_signum(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "hypot",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_hypot(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "exp",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_exp(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "exp2",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_exp2(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "exp_m1",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_exp_m1(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "ln",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_ln(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "max",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_max(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "min",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_min(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "avg",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_avg(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "to_degrees",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_to_degrees(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "to_radians",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_to_radians(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                        ]),
                    ),
                    (
                        "string",
                        HashMap::from([
                            (
                                "chars_count",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_chars_count(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "contains",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_contains(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "find",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_find(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "ends_with",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_ends_with(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "starts_with",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_starts_with(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "is_empty",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_is_empty(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "len",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_len(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "to_uppercase",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_to_uppercase(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "to_lowercase",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_to_lowercase(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "trim",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_trim(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "trim_end",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_trim_end(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "trim_start",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_trim_start(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                            (
                                "replace",
                                Box::new({
                                    let mut std = std.clone();
                                    move |name2: &Option<Token>| {
                                        std.load_replace(name2.clone());
                                    }
                                })
                                    as Box<dyn FnMut(&Option<Token>)>,
                            ),
                        ]),
                    ),
                    (
                        "vector",
                        HashMap::from([(
                            "vector",
                            Box::new({
                                let mut std = std.clone();
                                move |name2: &Option<Token>| {
                                    std.load_push(name2.clone());
                                }
                            }) as Box<dyn FnMut(&Option<Token>)>,
                        )]),
                    ),
                ],
            ),
        ])
    }

    pub fn load_std(&mut self, src: String, names: Vec<(Token, Option<Token>)>) {
        let parts: Vec<&str> = src.split("::").collect();
        let mut std = StdFunc::new(Rc::clone(&self.env), self.project.test);

        if parts[0] != "std" {
            raw(format!("standard library '{src}' doesn't exist").as_str());
            return;
        }

        if parts.len() < 2 {
            raw("standard library module not specified");
            return;
        }

        match self.std_map().get_mut(parts[1]) {
            Some(n) => match n.iter_mut().find(|(i, _)| parts[2] == *i) {
                Some(m) => {
                    if names.is_empty() {
                        match parts[1] {
                            "core" => match parts[2] {
                                "io" => std.load_core_io(),
                                "test" => std.load_core_test(),
                                _ => raw(format!(
                                    "std module '{}::{}' doesn't exist",
                                    parts[1], parts[2]
                                )
                                .as_str()),
                            },
                            "literal" => match parts[2] {
                                "number" => std.load_literal_number(),
                                "string" => std.load_literal_string(),
                                "vector" => std.load_literal_vector(),
                                _ => raw(format!(
                                    "std module '{}::{}' doesn't exist",
                                    parts[1], parts[2]
                                )
                                .as_str()),
                            },
                            _ => raw(format!("std module '{}' doesn't exist", parts[1]).as_str()),
                        }
                    }
                    names.iter().for_each(|(name1, name2)| {
                        if let Some(l) = m.1.get_mut(&name1.lexeme.as_str()) {
                            (l)(name2);
                        } else {
                            raw(format!(
                                "std module '{}::{}::{}' doesn't exist",
                                parts[1], parts[2], parts[3]
                            )
                            .as_str())
                        }
                    });
                }
                None => {
                    raw(format!("std module '{}::{}' doesn't exist", parts[1], parts[2]).as_str())
                }
            },
            None => raw(format!("std module '{}' doesn't exist", parts[1]).as_str()),
        }
    }
}
