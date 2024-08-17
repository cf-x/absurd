use core::panic;
use std::{any::Any, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        DeclrFuncType, FuncBody, FuncImpl, FuncValueType, LiteralType,
        Statement::{self, *},
        Token, Wrapper,
    },
    env::Env,
    expr::Expression,
};

#[derive(Debug)]
pub struct Interpreter {
    pub env: Env,
    pub specs: HashMap<String, LiteralType>,
    pub is_mod: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        let int = Self {
            env: Env::new(HashMap::new()),
            specs: HashMap::new(),
            is_mod: false,
        };
        // @todo pre-load std
        int.env.define(
            "print".to_string(),
            LiteralType::DeclrFunc(DeclrFuncType {
                name: "print".to_string(),
                arity: 1,
                func: Rc::new(Wrapper {
                    0: Box::new(|args: &[LiteralType]| {
                        println!("{:?}", args[0]);
                        LiteralType::Void
                    }),
                }),
            }),
        );
        int
    }
    pub fn new_with_env(env: Env) -> Self {
        let int = Self {
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
                Statement::Expression { expr } => {
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
                Func {
                    name,
                    value_type,
                    body,
                    params,
                    is_async,
                    is_pub,
                    is_impl,
                    is_mut,
                } => {
                    // @todo handle implementation,
                    // asynchroneity and param mutability
                    // let call = self.create_func(stmt);
                    // let func = LiteralType::Func(FuncValueType::Func(call));
                    self.env.define(name.lexeme.clone(), LiteralType::Any);
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
            let body: Vec<Statement> = match body {
                FuncBody::Statements(stmts) => stmts.iter().map(|x| x.clone()).collect(),
                _ => {
                    // @error invalid function body
                    panic!("invalid function body")
                }
            };

            FuncImpl {
                name: name.lexeme.clone(),
                value_type: value_type.clone(),
                body: FuncBody::Statements(body),
                params,
                is_async: *is_async,
                is_pub: *is_pub,
                is_impl: *is_impl,
                is_mut: *is_mut,
                env: self.env.clone(),
            }
        } else {
            // @error failed to create a function
            panic!(" failed to create a function");
        }
    }
}

pub fn run_func(func: FuncImpl, args: &Vec<Expression>, env: Env) -> LiteralType {
    if args.len() != func.params.len() {
        // @error invalid number of arguments
        panic!("invalid number of arguments")
    }
    let mut arg_values = vec![];
    for arg in args {
        arg_values.push(arg.eval(env.clone()));
    }
    let func_env = func.env.enclose();
    for (i, val) in arg_values.iter().enumerate() {
        if i < func.params.len() {
            // @todo type check arguments
            func_env.define(func.params[i].0.lexeme.clone(), val.clone());
        } else {
            // @error invalid number of arguments
            panic!("invalid number of arguments")
        }
    }
    let mut int = Interpreter::new_with_env(func_env);

    match func.body {
        FuncBody::Statements(body) => {
            for stmt in body {
                int.interpret(vec![&stmt]);
                let val = int.specs.get("return");
                if val.is_some() {
                    let v = val.unwrap().clone();
                    // @todo add output type checking
                    return v;
                }
            }
        }
        _ => {
            // @error invalid function body
            panic!("invalid function body")
        }
    }

    if func.value_type.lexeme != "void" {
        // @error missing return statement
        panic!("missing return statement")
    }
    LiteralType::Null
}
