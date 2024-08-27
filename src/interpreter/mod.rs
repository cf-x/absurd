pub mod env;
pub mod expr;
pub mod types;
use env::{Env, FuncKind, VarKind};
use expr::Expression;
pub mod load_std;
use crate::resolver::type_check;
use crate::std::core::io::StdCoreIo;
use crate::utils::bundler::interpreter_mod;
use crate::utils::errors::{Error, ErrorCode::*};
use crate::utils::manifest::Project;
use crate::{
    ast::{
        FuncBody, FuncImpl, FuncValueType, LiteralType,
        Statement::{self, *},
        Token,
    },
    utils::errors::raw,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::rc::Rc;

#[derive(Debug)]
pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
    pub specs: Rc<RefCell<HashMap<String, LiteralType>>>,
    pub is_mod: bool,
    mod_src: Option<String>,
    error: Error,
    pub project: Project,
}

impl Interpreter {
    pub fn new(project: Project, error: Error) -> Self {
        let env = Rc::new(RefCell::new(Env::new(HashMap::new())));
        let int = Self {
            env: env.clone(),
            specs: Rc::new(RefCell::new(HashMap::new())),
            is_mod: false,
            mod_src: None,
            error,
            project: project.clone(),
        };

        if !project.clone().disable_std && project.clone().load_std {
            let mut std_core_io = StdCoreIo::new(env);
            std_core_io.load();
        }
        int
    }

    pub fn new_with_env(
        env: Rc<RefCell<Env>>,
        is_mod: bool,
        src: &str,
        mod_src: Option<String>,
    ) -> Self {
        let int = Self {
            env: env.clone(),
            specs: Rc::new(RefCell::new(HashMap::new())),
            is_mod,
            mod_src,
            error: Error::new(src, Project::new()),
            project: Project::new(),
        };
        if is_mod {
            let mut std_core_io = StdCoreIo::new(env);
            std_core_io.load();
        }
        int
    }

    pub fn interpret(&mut self, stmts: Vec<&Statement>) -> Rc<RefCell<Env>> {
        for stmt in stmts {
            match stmt {
                Statement::Expression { expr } => {
                    expr.eval(Rc::clone(&self.env));
                }
                Block { stmts } => {
                    if !self.is_mod {
                        let new_env = self.env.borrow().enclose();
                        let prev_env = Rc::clone(&self.env);
                        self.env = Rc::new(RefCell::new(new_env));
                        self.interpret(stmts.iter().map(|x| x).collect());
                        self.env = prev_env;
                    }
                }
                Var {
                    names,
                    value,
                    is_pub,
                    pub_names,
                    is_func,
                    is_mut,
                    ..
                } => match value {
                    Some(v) => {
                        if is_mut.clone() && !self.project.side_effects {
                            self.error
                                .throw(E0x415, names[0].line, names[0].pos, vec![]);
                        } else if is_pub.clone() && !self.project.side_effects {
                            self.error
                                .throw(E0x415, names[0].line, names[0].pos, vec![]);
                        }

                        if !self.is_mod {
                            if is_func.clone() {
                                if names.len() != 1 {
                                    self.error
                                        .throw(E0x401, names[0].line, names[0].pos, vec![]);
                                }
                                let call = self.create_func(stmt);
                                let func = LiteralType::Func(FuncValueType::Func(call.clone()));
                                let params = call
                                    .params
                                    .iter()
                                    .map(|(a, b)| (a.clone().lexeme, b.clone().lexeme))
                                    .collect();
                                self.env.borrow_mut().define_func(
                                    names[0].lexeme.clone(),
                                    func,
                                    FuncKind {
                                        params,
                                        is_async: call.is_async,
                                        is_pub: is_pub.clone(),
                                    },
                                );
                            } else {
                                let val = v.eval(self.env.clone());
                                for name in names {
                                    self.env.borrow_mut().define_var(
                                        name.lexeme.clone(),
                                        val.clone(),
                                        VarKind {
                                            is_pub: is_pub.clone(),
                                            is_mut: *is_mut,
                                            is_func: false,
                                        },
                                    );
                                }
                                if is_pub.clone() {
                                    for name in pub_names {
                                        self.env.borrow_mut().define_pub_var(
                                            name.lexeme.clone(),
                                            val.clone(),
                                            VarKind {
                                                is_pub: true,
                                                is_mut: *is_mut,
                                                is_func: false,
                                            },
                                        );
                                    }
                                }
                            }
                        } else if is_pub.clone() {
                            let val = v.eval(Rc::clone(&self.env));
                            for name in pub_names {
                                self.env.borrow_mut().define_mod_var(
                                    self.mod_src.clone().unwrap(),
                                    val.clone(),
                                    name.lexeme.clone(),
                                    VarKind {
                                        is_pub: true,
                                        is_mut: *is_mut,
                                        is_func: false,
                                    },
                                );
                            }
                        }
                    }
                    None => {
                        if is_pub.clone() {
                            self.error
                                .throw(E0x402, names[0].line, names[0].pos, vec![]);
                        }
                        let val = LiteralType::Null;
                        for name in names {
                            self.env.borrow_mut().define_var(
                                name.lexeme.clone(),
                                val.clone(),
                                VarKind {
                                    is_pub: false,
                                    is_mut: *is_mut,
                                    is_func: false,
                                },
                            );
                        }
                    }
                },
                Func {
                    name,
                    is_pub,
                    params,
                    is_async,
                    ..
                } => {
                    if is_pub.clone() && !self.project.side_effects {
                        self.error.throw(E0x415, name.line, name.pos, vec![]);
                    }

                    let call = self.create_func(stmt);
                    let func = LiteralType::Func(FuncValueType::Func(call));
                    let params: Vec<(String, String)> = params
                        .iter()
                        .map(|(a, b)| (a.clone().lexeme, b.clone().lexeme))
                        .collect();
                    if is_pub.clone() {
                        if self.is_mod {
                            self.env.borrow_mut().define_mod_func(
                                self.mod_src.clone().unwrap(),
                                func.clone(),
                                name.lexeme.clone(),
                                FuncKind {
                                    params: params.clone(),
                                    is_async: *is_async,
                                    is_pub: true,
                                },
                            );
                        } else {
                            self.env.borrow_mut().define_pub_func(
                                name.lexeme.clone(),
                                func.clone(),
                                FuncKind {
                                    params,
                                    is_async: *is_async,
                                    is_pub: true,
                                },
                            );
                        }
                    } else if !self.is_mod {
                        self.env.borrow_mut().define_func(
                            name.lexeme.clone(),
                            func,
                            FuncKind {
                                params,
                                is_async: *is_async,
                                is_pub: false,
                            },
                        );
                    }
                }
                If {
                    cond,
                    body,
                    else_branch,
                    else_if_branches,
                } => {
                    if !self.is_mod {
                        let val = cond.eval(Rc::clone(&self.env));
                        if val.is_truthy() {
                            self.interpret(body.iter().map(|x| x).collect());
                        } else {
                            let mut executed = false;
                            for (cond, body) in else_if_branches {
                                let val = cond.eval(Rc::clone(&self.env));
                                if val.is_truthy() {
                                    executed = true;
                                    self.interpret(body.iter().map(|x| x).collect());
                                    break;
                                }
                            }
                            if let Some(body) = else_branch {
                                if !executed {
                                    self.interpret(body.iter().map(|x| x).collect());
                                }
                            }
                        }
                    }
                }
                Return { expr } => {
                    let value = expr.eval(Rc::clone(&self.env));
                    self.specs.borrow_mut().insert("return".to_string(), value);
                }
                While { cond, body } => {
                    if !self.is_mod {
                        while cond.eval(Rc::clone(&self.env)).is_truthy() {
                            self.interpret(body.iter().map(|x| x).collect());
                            if self.specs.borrow().get("break").is_some() {
                                self.specs.borrow_mut().remove("break");
                                break;
                            }
                        }
                    }
                }
                Loop { iter, body } => {
                    if !self.is_mod {
                        match iter {
                            Some(i) => {
                                for _ in 0..i.clone() {
                                    self.interpret(body.iter().map(|x| x).collect());
                                    if self.specs.borrow().get("break").is_some() {
                                        self.specs.borrow_mut().remove("break");
                                        break;
                                    }
                                }
                            }
                            None => loop {
                                self.interpret(body.iter().map(|x| x).collect());
                                if self.specs.borrow().get("break").is_some() {
                                    self.specs.borrow_mut().remove("break");
                                    break;
                                }
                            },
                        }
                    }
                }
                Break {} => {
                    self.specs
                        .borrow_mut()
                        .insert("break".to_string(), LiteralType::Null);
                }
                Match {
                    cond,
                    cases,
                    def_case,
                } => {
                    if !self.is_mod {
                        let mut exec = false;
                        let val = cond.eval(Rc::clone(&self.env));
                        for (expr, body) in cases {
                            let v = expr.eval(Rc::clone(&self.env));
                            if v.type_name() == val.type_name() {
                                match body.clone() {
                                    FuncBody::Statements(s) => {
                                        if v.clone() == val.clone() {
                                            self.interpret(s.iter().map(|x| x).collect());
                                            exec = true;
                                            break;
                                        }
                                    }
                                    FuncBody::Expression(e) => {
                                        if v.clone() == val.clone() {
                                            self.interpret(vec![&Statement::Expression {
                                                expr: *e,
                                            }]);
                                            exec = true;
                                            break;
                                        }
                                    }
                                }
                            } else {
                                raw("invad type in match statement")
                            }
                        }
                        if !exec {
                            match def_case.clone() {
                                FuncBody::Statements(s) => {
                                    self.interpret(s.iter().map(|x| x).collect());
                                }
                                FuncBody::Expression(e) => {
                                    self.interpret(vec![&Statement::Expression { expr: *e }]);
                                }
                            }
                        }
                    }
                }
                Mod { src } => {
                    if !self.project.side_effects {
                        self.error.throw(E0x415, 0, (0, 0), vec![]);
                    }
                    let mut path = current_dir().expect("failed to get current directory");
                    path.push(src.trim_matches('"'));
                    let mut file = File::open(path).expect("failed to open a file");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("failed to read a file");
                    interpreter_mod(
                        contents.as_str(),
                        Some(src.to_string()),
                        self.env.clone(),
                        self.project.clone(),
                    );
                }
                Use { src, names, all } => {
                    if !self.project.side_effects {
                        self.error.throw(E0x415, 0, (0, 0), vec![]);
                    }

                    if src.clone().contains("::") {
                        self.load_std(
                            src.trim_matches('"').to_string().clone(),
                            names.clone(),
                            all.clone(),
                        );
                    } else {
                        let mod_vals = self.env.borrow().mod_vals.borrow().clone();
                        let vals = match mod_vals.get(src) {
                            Some(c) => c,
                            None => {
                                self.error.throw(E0x416, 0, (0, 0), vec![src.clone()]);
                                exit(1);
                            }
                        };

                        self.env.borrow_mut().mod_vals.borrow_mut().remove(src);

                        if *all {
                            for val in vals {
                                let (name, v) = val;
                                self.env
                                    .borrow_mut()
                                    .values
                                    .borrow_mut()
                                    .insert(name.clone(), v.clone());
                            }
                        } else {
                            for (name, alias) in names {
                                if let Some((_, v)) = vals.iter().find(|(n, _)| n == &name.lexeme) {
                                    let new_name =
                                        alias.as_ref().map_or(&name.lexeme, |t| &t.lexeme);
                                    self.env
                                        .borrow_mut()
                                        .values
                                        .borrow_mut()
                                        .insert(new_name.clone(), v.clone());
                                }
                            }
                        }
                    }
                }
                Enum { .. } => {
                    // @todo handle enum statements
                }
            }
        }
        self.env.clone()
    }

    fn create_func(&self, stmt: &Statement) -> FuncImpl {
        if let Func {
            name,
            value_type,
            body,
            params,
            is_async,
            is_pub,
        } = stmt
        {
            let params: Vec<(Token, Token)> = params
                .iter()
                .map(|(name, value_type)| (name.clone(), value_type.clone()))
                .collect();
            let body: Vec<Statement> = match body {
                FuncBody::Statements(stmts) => stmts.iter().map(|x| x.clone()).collect(),
                _ => {
                    self.error.throw(E0x403, name.line, name.pos, vec![]);
                    exit(1);
                }
            };

            FuncImpl {
                name: name.lexeme.clone(),
                value_type: value_type.clone(),
                body: FuncBody::Statements(body),
                params,
                is_async: *is_async,
                is_pub: *is_pub,
                env: Rc::clone(&self.env),
            }
        } else {
            self.error.throw(E0x404, 0, (0, 0), vec![]);
            exit(1);
        }
    }
}

pub fn run_func(func: FuncImpl, args: &Vec<Expression>, env: Rc<RefCell<Env>>) -> LiteralType {
    let error = Error::new("", Project::new());
    if args.len() != func.params.len() {
        error.throw(E0x405, 0, (0, 0), vec![]);
    }
    let mut arg_values = vec![];
    for arg in args {
        arg_values.push(arg.eval(Rc::clone(&env)));
    }
    let func_env = func.env.borrow_mut().enclose();
    let func_env = Rc::new(RefCell::new(func_env));

    for (i, val) in arg_values.iter().enumerate() {
        if i < func.params.len() {
            if !type_check(&func.value_type, &val) {
                error.throw(
                    E0x301,
                    0,
                    (0, 0),
                    vec![val.to_string(), arg_values[i].to_string()],
                );
            }
            let params = func
                .params
                .iter()
                .map(|(a, b)| (a.clone().lexeme, b.clone().lexeme))
                .collect();
            func_env.borrow_mut().define_func(
                func.params[i].0.lexeme.clone(),
                val.clone(),
                FuncKind {
                    params,
                    is_async: func.is_async,
                    is_pub: func.is_pub,
                },
            );
        } else {
            error.throw(E0x405, 0, (0, 0), vec![]);
        }
    }
    // @todo pass is_mod
    let mut int = Interpreter::new_with_env(func_env, false, "", None);

    match func.body {
        FuncBody::Statements(body) => {
            for stmt in body {
                int.interpret(vec![&stmt]);
                let val = {
                    let specs = int.specs.borrow();
                    specs.get("return").cloned()
                };

                if val.is_some() {
                    let v = val.unwrap().clone();
                    if !type_check(&func.value_type, &v) {
                        error.throw(
                            E0x301,
                            0,
                            (0, 0),
                            vec![func.value_type.clone().lexeme, v.to_string()],
                        );
                    }
                    return v;
                }
            }
        }
        _ => {
            error.throw(E0x403, 0, (0, 0), vec![]);
        }
    }

    if func.value_type.lexeme != "void" {
        error.throw(E0x406, 0, (0, 0), vec![]);
    }
    LiteralType::Null
}
