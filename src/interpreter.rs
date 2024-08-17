use std::{collections::HashMap, process::exit};

use crate::{
    ast::{
        FuncImpl, FuncValueType, LiteralType,
        Statement::{self, *},
        Token,
    },
    env::Env,
};

#[derive(Debug)]
pub struct Interpreter {
    pub env: Env,
    pub specs: HashMap<String, LiteralType>,
    pub is_mod: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        let /* mut */ int = Self {
            env: Env::new(HashMap::new()),
            specs: HashMap::new(),
            is_mod: false,
        };
        // @todo pre-load std
        int
    }
    pub fn new_with_env(env: Env) -> Self {
        let /* mut */ int = Self {
            env,
            specs: HashMap::new(),
            is_mod: false,
        };
        // @todo pre-load std
        int
    }
    pub fn interpret(&mut self, stmts: Vec<&Statement>) {
        for stmt in stmts {
            match stmt {
                Expression { expr } => {
                    expr.eval(self.env.clone());
                }
                Block { stmts } => {
                    let new_env = self.env.enclose();
                    let prev_env = self.env.clone();
                    self.env = new_env;
                    self.interpret(stmts.iter().map(|x| x).collect());
                    self.env = prev_env;
                }
                Var {
                    names,
                    value,
                    is_pub,
                    pub_names,
                    ..
                } => match value {
                    Some(v) => {
                        let val = v.eval(self.env.clone());
                        for name in names {
                            self.env.define(name.lexeme.clone(), val.clone());
                        }
                        if *is_pub {
                            for name in pub_names {
                                self.env.define_pub(name.lexeme.clone(), val.clone());
                            }
                        }
                    }
                    None => {
                        // @todo handle null value
                    } // @todo handle functions
                },
                Func { name, .. } => {
                    // @todo handle implementation,
                    // asynchroneity and param mutability
                    let call = self.create_func(stmt);
                    let func = LiteralType::Func(FuncValueType::Func(call));
                    self.env.define(name.lexeme.clone(), func);
                }
                If { .. } => {
                    // @todo handle if statements
                }
                Return { expr } => {
                    let value = expr.eval(self.env.clone());
                    self.specs.insert("return".to_string(), value);
                }
                While { .. } => {}
                Loop { .. } => {}
                Break {} => {
                    self.specs.insert("break".to_string(), LiteralType::Null);
                }
                Match { .. } => {
                    // @todo handle match statements
                }
                Mod { .. } => {
                    // @todo handle mod statements
                }
                Use { .. } => {
                    // @todo handle use statements
                }
                Struct { .. } => {
                    // @todo handle struct statements
                }
                Impl { .. } => {
                    // @todo handle impl statements
                }
                Enum { .. } => {
                    // @todo handle enum statements
                }
            }
        }
    }
    fn create_func(&self, stmt: &Statement) -> FuncImpl {
        if let Func {
            name,
            value_type,
            body,
            params,
            is_async,
            is_pub,
            is_impl,
            is_mut,
        } = stmt
        {
            let params: Vec<(Token, Token)> = params
                .iter()
                .map(|(name, value_type)| (name.clone(), value_type.clone()))
                .collect();
            FuncImpl {
                name: name.lexeme.clone(),
                value_type: value_type.clone(),
                body: body.clone(),
                params,
                is_async: *is_async,
                is_pub: *is_pub,
                is_impl: *is_impl,
                is_mut: *is_mut,
                env: self.env.clone(),
            }
        } else {
            // @error failed to create a function
            exit(1);
        }
    }
}
