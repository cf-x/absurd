pub mod env;
pub mod expr;
pub mod types;
use crate::ast::{
    Destruct, FuncBody, FuncImpl, LiteralKind, LiteralType,
    Statement::{self, *},
    Token, TokenType,
};
use crate::bundler::interpreter_mod;
use crate::errors::{raw, Error, ErrorCode::*};
use crate::interpreter::types::type_check;
use crate::manifest::Project;
use crate::std::StdFunc;
use env::{Env, FuncKind, ValueKind, VarKind};
use expr::Expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command, Stdio};
use std::rc::Rc;
use types::TypeKind;

#[derive(Debug)]
pub struct Interpreter {
    /// interpreter envrironment
    pub env: Rc<RefCell<Env>>,
    /// project settings, will be moved to the env
    pub project: Project,
    /// specials like "return" and "breal"
    specs: Rc<RefCell<HashMap<String, LiteralType>>>,
    /// if interpreter is a module
    is_mod: bool,
    /// module source
    mod_src: Option<String>,
    /// error handler, will be moved to the env
    error: Error,
    /// order of the current statement
    order: usize,
}

impl Interpreter {
    /// initialize the Interpreter
    pub fn new(project: Project, error: Error) -> Self {
        let int = Self {
            env: Rc::new(RefCell::new(Env::new(HashMap::new()))),
            project: project.clone(),
            specs: Rc::new(RefCell::new(HashMap::new())),
            is_mod: false,
            mod_src: None,
            error,
            order: 0,
        };
        // load std::core::io
        if !project.clone().disable_std && project.clone().load_std {
            let mut std_core_io = StdFunc::new(Rc::clone(&int.env), int.project.test);
            std_core_io.load_core_io();
        }
        int
    }

    pub fn new_with_env(
        env: Rc<RefCell<Env>>,
        is_mod: bool,
        src: &str,
        mod_src: Option<String>,
        order: usize,
    ) -> Self {
        let int = Self {
            env: Rc::clone(&env),
            specs: Rc::new(RefCell::new(HashMap::new())),
            is_mod,
            mod_src,
            error: Error::new(src, Project::new()),
            project: Project::new(),
            order,
        };
        // load std::core::io if interpreter runs in the module
        if is_mod {
            let mut std_core_io = StdFunc::new(env, false);
            std_core_io.load_core_io();
        }
        int
    }

    /// iterates of statements and executes each statement
    /// set order to 0 if statement is first class
    /// set order to 1 if statement is inside the block
    pub fn interpret(&mut self, stmts: Vec<&Statement>, order: usize) -> Rc<RefCell<Env>> {
        self.order = order;
        for stmt in stmts {
            match stmt {
                Statement::Expression { expr } => {
                    expr.eval(Rc::clone(&self.env));
                }
                Block { stmts } => self.block(stmts.clone()),
                Var { .. } => self.variable(stmt),
                Func { .. } => self.func(stmt),
                Return { expr } => {
                    let value = expr.eval(Rc::clone(&self.env));
                    self.specs.borrow_mut().insert("return".to_string(), value);
                }
                If { .. } => self.ifs(stmt),
                Loop { iter, body } => self.loops(iter.clone(), body.clone()),
                While { cond, body } => self.whiles(cond, body.clone()),
                For { .. } => self.fors(stmt),
                Break {} => {
                    self.specs
                        .borrow_mut()
                        .insert("break".to_string(), LiteralType::Null);
                }
                Match {
                    cond,
                    cases,
                    def_case,
                } => self.matchs(cond, cases.clone(), def_case),
                Enum {
                    name,
                    is_pub,
                    items,
                } => self.enums(name, *is_pub, items),
                Type {
                    name,
                    value,
                    is_pub,
                } => self.types(name, value, *is_pub),
                Statement::Record { .. } => self.record(stmt),
                Mod { src, name } => self.mods(src, name.clone()),
                Use { src, names, all } => self.uses(src, names.clone(), *all),
                Sh { cmd } => self.sh(cmd),
            }
        }
        Rc::clone(&self.env)
    }

    fn block(&mut self, stmts: Vec<Statement>) {
        let new_env = self.env.borrow_mut().enclose();
        let prev_env = Rc::clone(&self.env);
        self.env = Rc::new(RefCell::new(new_env));
        self.interpret(stmts.iter().map(|x| x).collect(), 1);
        self.env = prev_env;
    }

    fn variable(&mut self, stmt: &Statement) {
        if let Statement::Var {
            names,
            destruct,
            value_type,
            value,
            is_mut,
            is_pub,
            pub_names,
            is_func,
        } = stmt
        {
            if value.is_some() {
                // disable mutability in side effects
                if is_mut.clone() && !self.project.side_effects {
                    self.error
                        .throw(E0x415, names[0].line, names[0].pos, vec![]);
                } else
                // disable publicity in side effects
                if is_pub.clone() && !self.project.side_effects {
                    self.error
                        .throw(E0x415, names[0].line, names[0].pos, vec![]);
                }

                let vl = value.clone().unwrap().eval(Rc::clone(&self.env));
                // @todo replace value_type with option<Token>
                // don't type check during type inference
                if value_type.token != TokenType::Null {
                    if !type_check(&value_type, &vl, &self.env) {
                        self.error.throw(
                            E0x301,
                            names[0].line,
                            names[0].pos,
                            vec![value_type.clone().lexeme, vl.to_string()],
                        );
                    }
                }

                // hande variables in modules
                if self.is_mod && self.order == 0 && *is_pub {
                    // define variables in the module
                    let val = value.as_ref().unwrap().eval(Rc::clone(&self.env));
                    pub_names.iter().for_each(|name| {
                        self.env.borrow_mut().define_mod_var(
                            self.mod_src.clone().unwrap(),
                            val.clone(),
                            name.lexeme.clone(),
                            VarKind {
                                is_pub: true,
                                is_mut: *is_mut,
                                is_func: false,
                                value_type: value_type.clone(),
                            },
                        )
                    });
                }

                // handle callbacks in the normal env
                if !self.is_mod && *is_func {
                    // callbacks must have one name
                    if names.len() != 1 {
                        self.error
                            .throw(E0x401, names[0].line, names[0].pos, vec![]);
                    }

                    // callbacks can't mutate
                    if *is_mut {
                        raw("functions can't be mutable");
                    }

                    // create and define the function
                    let call = self.create_func(stmt);
                    let func = LiteralType::Func(call.clone());
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
                }
                let var_kind = VarKind {
                    is_pub: is_pub.clone(),
                    is_mut: *is_mut,
                    is_func: *is_func,
                    value_type: value_type.clone(),
                };
                // hande normal variable
                if !self.is_mod {
                    let val = value.clone().unwrap().eval(Rc::clone(&self.env));

                    // handle the name based on the value type for destructuring
                    for (index, name) in names.clone().iter().enumerate() {
                        match val.clone() {
                            LiteralType::Vec(entries) => {
                                if destruct.is_some()
                                    && destruct.clone().unwrap() == Destruct::Vector
                                {
                                    // get the nth entry of the vector for the each name
                                    let entry = entries
                                        .get(index)
                                        .expect("failed to destructure a vector")
                                        .clone();

                                    // hadnel publicty
                                    if *is_pub {
                                        // @todo destructure pub(names) as well
                                        self.env.borrow_mut().define_pub_var(
                                            name.lexeme.clone(),
                                            entry,
                                            var_kind.clone(),
                                        );
                                    } else {
                                        self.env.borrow_mut().define_var(
                                            name.lexeme.clone(),
                                            entry,
                                            var_kind.clone(),
                                        );
                                    }
                                }
                            }
                            LiteralType::Tuple(entries) => {
                                if destruct.is_some()
                                    && Destruct::Tuple == destruct.clone().unwrap()
                                {
                                    // get the nth entry of the vector for each name
                                    let entry = entries
                                        .get(index)
                                        .expect("failed to destructure a tuple")
                                        .clone();

                                    // handle publicty
                                    if *is_pub {
                                        self.env.borrow_mut().define_pub_var(
                                            name.lexeme.clone(),
                                            entry,
                                            var_kind.clone(),
                                        );
                                    } else {
                                        self.env.borrow_mut().define_var(
                                            name.lexeme.clone(),
                                            entry,
                                            var_kind.clone(),
                                        );
                                    }
                                }
                            }
                            LiteralType::Record(entries) => {
                                if destruct.is_some() {
                                    if let Destruct::Record = destruct.clone().unwrap() {
                                        // get the nth entry of the vector for each name
                                        // @todo get the entry based on the name
                                        let entry = entries
                                            .get(index)
                                            .expect("failed to destructure an array")
                                            .clone();
                                        let entry = entry.1.eval(Rc::clone(&self.env));
                                        // handle publicty
                                        if *is_pub {
                                            self.env.borrow_mut().define_pub_var(
                                                name.lexeme.clone(),
                                                entry,
                                                var_kind.clone(),
                                            );
                                        } else {
                                            self.env.borrow_mut().define_var(
                                                name.lexeme.clone(),
                                                entry,
                                                var_kind.clone(),
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }

                        // normal definition
                        if destruct.is_none() {
                            self.env.borrow_mut().define_var(
                                name.lexeme.clone(),
                                val.clone(),
                                var_kind.clone(),
                            );
                        }
                    }

                    // public definition
                    if *is_pub {
                        for name in pub_names {
                            self.env.borrow_mut().define_pub_var(
                                name.lexeme.clone(),
                                val.clone(),
                                var_kind.clone(),
                            );
                        }
                    }
                }
            } else {
                // handle empty variables

                // can't publish empty variables
                if is_pub.clone() {
                    self.error
                        .throw(E0x402, names[0].line, names[0].pos, vec![]);
                }

                // define every name in the variable
                names.iter().for_each(|name| {
                    self.env.borrow_mut().define_var(
                        name.lexeme.clone(),
                        LiteralType::Null,
                        VarKind {
                            is_pub: false,
                            is_mut: *is_mut,
                            is_func: false,
                            value_type: value_type.clone(),
                        },
                    )
                });
            }
        }
    }

    fn func(&mut self, stmt: &Statement) {
        if let Statement::Func {
            name,
            body,
            params,
            is_async,
            is_pub,
            ..
        } = stmt
        {
            let is_async = *is_async;
            let is_pub = *is_pub;

            // evaluate return statement
            match body {
                FuncBody::Statements(statements) => {
                    for statement in statements {
                        if let Statement::Return { expr } = statement {
                            expr.eval(Rc::clone(&self.env));
                        }
                    }
                }
                // for short functions
                FuncBody::Expression(expression) => {
                    expression.eval(Rc::clone(&self.env));
                }
            }

            // publicity is disabled in side effects
            if is_pub && !self.project.side_effects {
                self.error.throw(E0x415, name.line, name.pos, vec![]);
            }

            // create a function
            let call = self.create_func(stmt);
            let func = LiteralType::Func(call);

            // convert param Tokens to the strings
            let params: Vec<(String, String)> = params
                .iter()
                .map(|(a, b)| (a.lexeme.clone(), b.lexeme.clone()))
                .collect();

            let func_kind = FuncKind {
                params,
                is_async,
                is_pub,
            };
            // hande public function
            if is_pub && !self.is_mod {
                self.env.borrow_mut().define_pub_func(
                    name.lexeme.clone(),
                    func.clone(),
                    func_kind.clone(),
                );
            } else
            //handle public function in the module
            if is_pub && self.is_mod && self.order == 0 {
                self.env.borrow_mut().define_mod_func(
                    self.mod_src.clone().unwrap(),
                    func.clone(),
                    name.lexeme.clone(),
                    func_kind.clone(),
                );
            } else
            // handle normal functions
            if !self.is_mod {
                self.env
                    .borrow_mut()
                    .define_func(name.lexeme.clone(), func, func_kind);
            }
        }
    }

    fn ifs(&mut self, stmt: &Statement) {
        if let Statement::If {
            cond,
            body,
            else_if_branches,
            else_branch,
        } = stmt
        {
            if !self.is_mod {
                let val = cond.eval(Rc::clone(&self.env));
                // if condition is true, execute the body
                if val.is_truthy() {
                    self.interpret(body.iter().map(|x| x).collect(), 1);
                } else {
                    let mut executed = false;
                    // check elif branches
                    for (cond, body) in else_if_branches {
                        let val = cond.eval(Rc::clone(&self.env));
                        if val.is_truthy() {
                            executed = true;
                            self.interpret(body.iter().map(|x| x).collect(), 1);
                            break;
                        }
                    }
                    // if non of the elif branches were executed, execute else branch if there
                    if let Some(body) = else_branch {
                        if !executed {
                            self.interpret(body.iter().map(|x| x).collect(), 1);
                        }
                    }
                }
            }
        }
    }

    fn loops(&mut self, iter: Option<usize>, body: Vec<Statement>) {
        if !self.is_mod {
            match iter {
                // explicit iterations
                Some(i) => {
                    for _ in 0..i.clone() {
                        self.interpret(body.iter().map(|x| x).collect(), 1);
                        if self.specs.borrow_mut().get("break").is_some() {
                            self.specs.borrow_mut().remove("break");
                            break;
                        }
                    }
                }
                // infinite loop
                None => loop {
                    self.interpret(body.iter().map(|x| x).collect(), 1);
                    if self.specs.borrow_mut().get("break").is_some() {
                        self.specs.borrow_mut().remove("break");
                        break;
                    }
                },
            }
        }
    }

    fn whiles(&mut self, cond: &Expression, body: Vec<Statement>) {
        if !self.is_mod {
            // execute code while the condition is truthy
            while cond.eval(Rc::clone(&self.env)).is_truthy() {
                self.interpret(body.iter().map(|x| x).collect(), 1);
                if self.specs.borrow_mut().get("break").is_some() {
                    self.specs.borrow_mut().remove("break");
                    break;
                }
            }
        }
    }

    fn fors(&mut self, stmt: &Statement) {
        if let Statement::For {
            iterator,
            index,
            expr,
            body,
        } = stmt
        {
            if !self.is_mod {
                // transform expression into the literal
                // iterations over vectors are supported yet
                // @todo: tupple, record
                let values = if let LiteralType::Vec(items) = expr.eval(Rc::clone(&self.env)) {
                    items.clone()
                } else {
                    vec![]
                };

                // iterate between values and define arguments
                for (id, iter) in values.iter().enumerate() {
                    if let Some(token) = index {
                        self.env.borrow_mut().define_var(
                            token.clone().lexeme,
                            LiteralType::Number(id as f32),
                            VarKind {
                                is_pub: false,
                                is_mut: false,
                                is_func: false,
                                value_type: token.clone(),
                            },
                        );
                    }

                    self.env.borrow_mut().define_var(
                        iterator.clone().lexeme,
                        iter.clone(),
                        VarKind {
                            is_pub: false,
                            is_mut: false,
                            is_func: false,
                            value_type: iterator.clone(),
                        },
                    );

                    self.interpret(body.iter().map(|x| x).collect(), 1);
                    if self.specs.borrow_mut().get("break").is_some() {
                        self.specs.borrow_mut().remove("break");
                        break;
                    }
                }
                // destroy arguments after the iteration is over
                if let Some(token) = index {
                    self.env.borrow_mut().remove(token.clone().lexeme);
                }
                self.env.borrow_mut().remove(iterator.clone().lexeme);
            }
        }
    }

    fn matchs(
        &mut self,
        cond: &Expression,
        cases: Vec<(Expression, FuncBody)>,
        def_case: &FuncBody,
    ) {
        if !self.is_mod {
            // if case has been executed
            let mut exec = false;
            let condition = cond.eval(Rc::clone(&self.env));

            // number, string, char matching:
            // - require def case
            // - check if case_cond equals condition

            // bool matching:
            // - require true and false cases
            // - if one of them or none of them are there, require def_case

            // null, void, any matching:
            // - require def case
            // - don't allow any other case

            // vector, tuple, record, .., other matching:
            // - don't allow pattern matching
            match condition.clone() {
                LiteralType::Enum { name, .. } => {
                    // enum collections aren't allowed
                    if name.token == TokenType::Null {
                        raw("please specify the enum name, you are trying to match");
                    }

                    for (expr, body) in cases {
                        let body = match body {
                            FuncBody::Expression(ref expr) => {
                                vec![Statement::Expression {
                                    expr: *expr.clone(),
                                }]
                            }

                            FuncBody::Statements(ref stmts) => stmts.clone(),
                        };
                        let body = body.iter().collect();
                        let expr_lit = expr.eval(Rc::clone(&self.env));
                        // check if expression is enum
                        if let LiteralType::Enum { .. } = expr_lit {
                            // execute the body if case matches
                            if self
                                .enum_equality(expr.eval(Rc::clone(&self.env)), condition.clone())
                            {
                                self.interpret(body, 1);
                                exec = true;
                                break;
                            }
                        } else {
                            raw(format!(
                                "expected enum in the match condition, but received {}",
                                expr.eval(Rc::clone(&self.env)).type_name()
                            )
                            .as_str())
                        }
                    }
                }
                _ => raw(format!("pattern matching for '{:?}' isn't allowed", condition).as_str()),
            }

            if !exec {
                match def_case.clone() {
                    FuncBody::Statements(s) => {
                        self.interpret(s.iter().map(|x| x).collect(), 1);
                    }
                    FuncBody::Expression(e) => {
                        self.interpret(vec![&Statement::Expression { expr: *e }], 1);
                    }
                }
            }
        }
    }

    fn enum_equality(&mut self, lhs: LiteralType, rhs: LiteralType) -> bool {
        if let LiteralType::Enum {
            parent: lhs_par,
            name: lhs_name,
            ..
        } = lhs
        {
            if let LiteralType::Enum {
                parent: rhs_par,
                name: rhs_name,
                ..
            } = rhs
            {
                if lhs_par.lexeme == rhs_par.lexeme && lhs_name.lexeme == rhs_name.lexeme {
                    return true;
                }
            }
        }
        false
    }

    fn enums(&mut self, name: &Token, is_pub: bool, items: &Vec<(Token, Option<Token>)>) {
        // handle public enums in the module
        if is_pub && self.is_mod && self.order == 0 {
            self.env.borrow_mut().define_mod_enum(
                self.mod_src.clone().unwrap(),
                LiteralType::Void,
                name.clone().lexeme,
                items.clone(),
            );
        } else
        // handle public enums
        if is_pub {
            self.env
                .borrow_mut()
                .define_pub_enum(name.clone().lexeme, items.clone());
        } else
        // handle normal enums
        {
            self.env
                .borrow_mut()
                .define_enum(name.clone().lexeme, items.clone());
        }
    }

    fn types(&mut self, name: &Token, value: &Token, is_pub: bool) {
        // handle public types in modules
        if is_pub && self.is_mod && self.order == 0 {
            self.env.borrow_mut().define_mod_type(
                self.mod_src.clone().unwrap(),
                LiteralType::Void,
                name.clone().lexeme,
                value.clone(),
            );
        } else
        // handle public types
        if is_pub {
            self.env
                .borrow_mut()
                .define_pub_type(name.clone().lexeme, value.clone());
        } else
        // handle normal types
        {
            self.env
                .borrow_mut()
                .define_type(name.clone().lexeme, value.clone());
        }
    }

    fn record(&mut self, stmt: &Statement) {
        if let Statement::Record {
            name,
            extends: _,
            is_strict: _,
            fields,
        } = stmt
        {
            let fields: Vec<(Token, TypeKind)> = fields
                .iter()
                .map(|f| (f.name.clone(), f.value.clone().token_to_typekind()))
                .collect();

            let value = Some(LiteralKind::Type(Box::new(TypeKind::Record {
                fields: fields.clone(),
            })));

            let s: String = fields
                .iter()
                .map(|(i, v)| format!("{}: {}, ", i.lexeme.clone(), v.clone()))
                .collect();

            let value = Token {
                token: TokenType::Type,
                lexeme: format!("{{ {}}}", s),
                value,
                line: 0,
                pos: (0, 0),
            };

            self.env
                .borrow_mut()
                .define_type(name.clone().lexeme, value.clone());
        }
    }

    fn sh(&mut self, cmd: &String) {
        let cmd = cmd.trim_matches('"');
        let output = match Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(c) => c,
            Err(e) => {
                raw(format!("sh error: {}", e).as_str());
                exit(1)
            }
        };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            raw(format!("sh error: {}", stderr).as_str());
        }
    }

    /// creates FuncImpl from function statement
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
            let body: FuncBody = match body {
                FuncBody::Statements(stmts) => {
                    FuncBody::Statements(stmts.iter().map(|x| x.clone()).collect())
                }
                FuncBody::Expression(e) => FuncBody::Expression(e.clone()),
            };

            FuncImpl {
                name: name.lexeme.clone(),
                value_type: value_type.clone(),
                body,
                params,
                is_async: *is_async,
                is_pub: *is_pub,
                env: Rc::clone(&self.env),
            }
        } else if let Var { value, is_func, .. } = stmt {
            if !is_func.clone() {
                self.error.throw(E0x404, 0, (0, 0), vec![]);
                exit(1);
            }
            let func = value.clone().unwrap();
            if let Expression::Func {
                id: _,
                name,
                value_type,
                body,
                params,
                is_async,
                is_pub,
            } = func
            {
                let params: Vec<(Token, Token)> = params
                    .iter()
                    .map(|(name, value_type)| (name.clone(), value_type.clone()))
                    .collect();
                let body: Vec<Statement> = match body {
                    FuncBody::Statements(stmts) => stmts.iter().map(|x| x.clone()).collect(),
                    FuncBody::Expression(e) => vec![Statement::Expression { expr: *e.clone() }],
                };

                return FuncImpl {
                    name: name.lexeme.clone(),
                    value_type: value_type.clone(),
                    body: FuncBody::Statements(body),
                    params,
                    is_async,
                    is_pub,
                    env: Rc::clone(&self.env),
                };
            }
            self.error.throw(E0x404, 0, (0, 0), vec![]);
            exit(1);
        } else {
            self.error.throw(E0x404, 0, (0, 0), vec![]);
            exit(1);
        }
    }

    fn mods(&mut self, src: &String, name: Option<String>) {
        if !self.project.side_effects {
            self.error.throw(E0x415, 0, (0, 0), vec![]);
        }
        let mut path = current_dir().expect("failed to get current directory");
        path.push(src.trim_matches('"'));
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(f) => {
                raw(format!("failed to opan a file: {}", f).as_str());
                exit(0);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read a file");
        let name = if name.is_some() {
            format!("\"{}\"", name.clone().unwrap())
        } else {
            src.to_string()
        };
        interpreter_mod(
            contents.as_str(),
            Some(name),
            Rc::clone(&self.env),
            self.project.clone(),
        );
    }

    fn uses(&mut self, src: &String, names: Vec<(Token, Option<Token>)>, all: bool) {
        if !self.project.side_effects {
            self.error.throw(E0x415, 0, (0, 0), vec![]);
        }

        if src.clone().contains("::") {
            self.load_std(src.trim_matches('"').to_string().clone(), names.clone());
        } else {
            let mod_vals = self.env.borrow_mut().mod_vals.borrow_mut().clone();
            let vals = match mod_vals.get(src) {
                Some(c) => c,
                None => {
                    self.error.throw(E0x416, 0, (0, 0), vec![src.clone()]);
                    exit(1);
                }
            };

            self.env.borrow_mut().mod_vals.borrow_mut().remove(src);

            if all {
                for val in vals {
                    let (name, v) = val;

                    if let LiteralType::Void = v.value {
                        if let ValueKind::Type(t) = v.kind.clone() {
                            self.env
                                .borrow_mut()
                                .type_values
                                .borrow_mut()
                                .insert(name.clone(), t);
                        }
                    } else {
                        self.env
                            .borrow_mut()
                            .values
                            .borrow_mut()
                            .insert(name.clone(), v.clone());
                    }
                }
            } else {
                for (name, alias) in names {
                    if let Some((_, v)) = vals.iter().find(|(n, _)| n == &name.lexeme) {
                        let new_name = alias.as_ref().map_or(&name.lexeme, |t| &t.lexeme);
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
}

pub fn run_func(func: FuncImpl, args: &[Expression], env: Rc<RefCell<Env>>) -> LiteralType {
    let error = Error::new("", Project::new());
    if args.len() != func.params.len() {
        error.throw(E0x405, 0, (0, 0), vec![]);
    }

    let mut arg_values = vec![];
    for (i, arg) in args.iter().enumerate() {
        let arg_lit = arg.eval(Rc::clone(&env));
        if !type_check(&func.params.iter().nth(i).unwrap().1, &arg_lit, &env) {
            error.throw(
                E0x301,
                0,
                (0, 0),
                vec![
                    func.params.iter().nth(i).unwrap().1.lexeme.clone(),
                    arg_lit.to_string(),
                ],
            );
        }
        arg_values.push(arg_lit);
    }
    let func_env = func.env.borrow_mut().enclose();
    let func_env = Rc::new(RefCell::new(func_env));

    for (i, val) in arg_values.iter().enumerate() {
        if i < func.params.len() {
            if !type_check(&func.value_type, &val, &env) {
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

    let mut int = Interpreter::new_with_env(Rc::clone(&func_env), false, "", None, 1);
    match func.body {
        FuncBody::Statements(body) => {
            for stmt in body.clone() {
                int.interpret(vec![&stmt], 1);
                let mut val = {
                    let specs = int.specs.borrow_mut();
                    specs.get("return").cloned()
                };
                if let Statement::Expression { expr } = body.first().unwrap() {
                    val = Some(expr.eval(Rc::clone(&env)));
                }

                if val.is_some() {
                    let v = val.clone().unwrap().clone();
                    if !type_check(&func.value_type, &v, &env) {
                        // error.throw(
                        //     E0x301,
                        //     0,
                        //     (0, 0),
                        //     vec![func.value_type.clone().lexeme, v.to_string()],
                        // );
                    }
                    return v;
                }
            }
        }
        FuncBody::Expression(expr) => {
            let val = expr.eval(Rc::clone(&func_env));
            if !type_check(&func.value_type, &val, &env) {
                error.throw(
                    E0x301,
                    0,
                    (0, 0),
                    vec![func.value_type.clone().lexeme, val.to_string()],
                );
            }
            return val;
        }
    }
    if func.value_type.lexeme != "void" {
        error.throw(E0x406, 0, (0, 0), vec![]);
    }
    LiteralType::Null
}

// @todo better organized statements
