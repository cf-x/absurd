use crate::ast::{FuncBody, LiteralType, Statement, Token};
use crate::env::Env;
use crate::expr::Expression;
use core::panic;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Bool {
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Resolver {
    locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    is_crnt_fnc: Bool,
    is_crnt_loop: Bool,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            locals: HashMap::new(),
            scopes: Vec::new(),
            is_crnt_fnc: Bool::False,
            is_crnt_loop: Bool::False,
        }
    }
    pub fn resolve(&mut self, stmts: &Vec<&Statement>, env: &mut Env) -> HashMap<usize, usize> {
        self.resolve_stmts(stmts, env);
        self.locals.clone()
    }

    fn resolve_stmts(&mut self, stmts: &Vec<&Statement>, env: &mut Env) {
        for stmt in stmts {
            self.resolve_int(stmt, env)
        }
    }

    fn resolve_int(&mut self, stmt: &Statement, env: &mut Env) {
        match stmt {
            Statement::If { .. } => self.resolve_if_stmt(stmt, env),
            Statement::Block { .. } => self.resolve_block(stmt, env),
            Statement::Break { .. } => self.resolve_break_stmt(stmt, env),
            Statement::Enum { .. } => self.resolve_enum_stmt(stmt, env),
            Statement::Expression { expr } => self.resolve_expr(expr, env),
            Statement::Func { .. } => self.resolve_func_stmt(stmt, env),
            Statement::Impl { .. } => self.resolve_impl_stmt(stmt, env),
            Statement::Loop { .. } => self.resolve_loop_stmt(stmt, env),
            Statement::Match { .. } => self.resolve_match_stmt(stmt, env),
            Statement::Mod { .. } => self.resolve_mod_stmt(stmt, env),
            Statement::Return { .. } => self.resolve_return_stmt(stmt, env),
            Statement::Struct { .. } => self.resolve_struct_stmt(stmt, env),
            Statement::Use { .. } => self.resolve_use_stmt(stmt, env),
            Statement::Var { .. } => self.resolve_var_stmt(stmt, env),
            Statement::While { .. } => self.resolve_while_stmt(stmt, env),
        }
    }

    fn resolve_struct_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Struct { .. } = stmt {
            // @todo
        }
    }

    fn resolve_use_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Use { names, .. } = stmt {
            for name in names {
                let (old, new) = name;
                if new.is_some() {
                    self.declare(new.as_ref().unwrap());
                    self.define(new.as_ref().unwrap());
                } else {
                    self.declare(old);
                    self.define(old);
                }
            }
        }
    }

    fn resolve_var_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Var {
            names,
            pub_names,
            value,
            ..
        } = stmt
        {
            for name in names {
                self.declare(name);
                if let Some(value) = value {
                    let val = (*value).eval(env.clone());
                    let _val_clone = val.clone();
                    // @todo type checking
                    self.resolve_expr(value, env);
                }
                self.define(name);
            }
            // @todo better handle public names
            for pub_name in pub_names {
                self.declare(pub_name);
                if let Some(value) = value {
                    self.resolve_expr(value, env);
                }
                self.define(pub_name);
            }
        }
    }

    fn resolve_while_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::While { body, cond } = stmt {
            let encl_loop = self.is_crnt_loop;
            self.resolve_expr(cond, env);
            self.is_crnt_loop = Bool::True;
            self.resolve_many(&body.iter().collect(), env);
            self.is_crnt_loop = encl_loop;
        }
    }

    fn resolve_break_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        match self.is_crnt_loop {
            Bool::True => {}
            Bool::False => {
                // @error break outside of loop
            }
        }
    }

    fn resolve_enum_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Enum { name, .. } = stmt {
            self.declare(name);
            self.define(name);
        }
    }

    fn resolve_func_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Func {
            // value_type,
            body,
            params,
            ..
        } = stmt
        {
            let encl_func = self.is_crnt_fnc;
            self.is_crnt_fnc = Bool::True;
            self.scope_start();
            for (parname, _partype) in params {
                // @todo handle type checking
                self.declare(parname);
                self.define(parname);
            }

            match body {
                FuncBody::Statements(body) => {
                    self.resolve_many(&body.iter().collect(), env);
                }
                _ => {
                    // @error invalid function return type
                }
            }
            // @todo handle value_type checking
            self.scope_end();
            self.is_crnt_fnc = encl_func;
        }
    }

    fn resolve_impl_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Impl { body, .. } = stmt {
            self.scope_start();
            self.resolve_many(&body.iter().collect(), env);
            self.scope_end();
        }
    }

    fn resolve_loop_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Loop { body, .. } = stmt {
            self.scope_start();
            let encl_loop = self.is_crnt_loop;
            self.is_crnt_loop = Bool::True;
            self.resolve_many(&body.iter().collect(), env);
            self.is_crnt_loop = encl_loop;
            self.scope_end();
        }
    }

    fn resolve_match_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Match {
            cond,
            def_case,
            cases,
        } = stmt
        {
            self.resolve_expr(cond, env);
            for (case, body) in cases {
                self.scope_start();
                self.resolve_expr(case, env);
                match body {
                    FuncBody::Statements(stmts) => {
                        self.resolve_many(&stmts.iter().collect(), env);
                    }
                    FuncBody::Expression(expr) => {
                        self.resolve_expr(expr, env);
                    }
                }
                self.scope_end();
            }

            match def_case {
                FuncBody::Statements(stmts) => {
                    if !stmts.is_empty() {
                        self.scope_start();
                        self.resolve_many(&stmts.iter().collect(), env);
                        self.scope_end();
                    }
                }
                FuncBody::Expression(expr) => {
                    self.resolve_expr(expr, env);
                }
            }
        }
    }

    fn resolve_mod_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::Mod { src } = stmt {
            env.define_mod(src.clone(), LiteralType::String(src.clone()));
        }
    }

    fn resolve_return_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        match self.is_crnt_fnc {
            Bool::True => {
                if let Statement::Return { expr } = stmt {
                    self.resolve_expr(expr, env);
                }
            }
            _ => {
                // @error return outside of function
            }
        }
    }

    fn resolve_if_stmt(&mut self, stmt: &Statement, env: &mut Env) {
        if let Statement::If {
            cond,
            body,
            else_if_branches,
            else_branch,
        } = stmt
        {
            self.resolve_expr(cond, env);
            self.scope_start();
            self.resolve_many(&body.iter().collect(), env);
            self.scope_end();
            for (elif_pred, elif_stmt) in else_if_branches {
                // @todo type cheking
                self.resolve_expr(elif_pred, env);
                self.scope_start();
                self.resolve_many(&elif_stmt.iter().collect(), env);
                self.scope_end();
            }
            if let Some(branch) = else_branch {
                self.scope_start();
                self.resolve_many(&branch.iter().collect(), env);
                self.scope_end();
            }
        }
    }

    fn resolve_block(&mut self, stmt: &Statement, env: &mut Env) {
        match stmt {
            Statement::Block { stmts } => {
                self.scope_start();
                self.resolve_many(&stmts.iter().collect(), env);
                self.scope_end();
            }
            _ => panic!("@error failed to resolve a block statement"),
        }
    }

    fn resolve_expr(&mut self, expr: &Expression, env: &mut Env) {
        match expr {
            Expression::Array { items, .. } => {
                for item in items {
                    // @todo resolve LiteralType
                }
            }
            Expression::Var { .. } => self.resolve_var_expr(expr),
            Expression::Call { name, args, .. } => {
                self.resolve_expr(name.as_ref(), env);
                for arg in args {
                    self.resolve_expr(arg, env)
                }
            }
            Expression::Func {
                value_type,
                body,
                params,
                is_async,
                is_pub,
                ..
            } => self.resolve_func_expr(value_type, body, params, is_async, is_pub, env),
            Expression::Await { expr, .. } => self.resolve_expr(expr, env),
            Expression::Unary { left, .. } => self.resolve_expr(left, env),
            Expression::Value { .. } => {}
            Expression::Binary { left, right, .. } => {
                self.resolve_expr(left, env);
                self.resolve_expr(right, env);
            }
            Expression::Grouping { expression, .. } => self.resolve_expr(expression, env),
        }
    }

    fn resolve_func_expr(
        &mut self,
        value_type: &Token,
        body: &FuncBody,
        params: &Vec<(Token, Token)>,
        is_async: &bool,
        is_pub: &bool,
        env: &mut Env,
    ) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = Bool::True;
        self.scope_start();
        for (parname, _partype) in params {
            // @todo handle type checking
            self.declare(parname);
            self.define(parname);
        }

        match body {
            FuncBody::Statements(body) => {
                self.resolve_many(&body.iter().collect(), env);
            }
            _ => {
                // @error invalid function return type
            }
        }
        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn resolve_var_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Var { name, .. } => {
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes[self.scopes.len() - 1].get(&name.lexeme) {
                        panic!("@error: failed to read local variable")
                    }
                }
            }
            Expression::Call { name, .. } => match name.as_ref() {
                Expression::Var { name, .. } => self.resolve_local(&name, expr.clone().id()),
                _ => {
                    panic!("@error: failed to resolve variable")
                }
            },
            _ => {
                panic!("@error: failed to resolve variable");
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        let s = self.scopes.len();
        // @todo: add shadowing
        if self.scopes.is_empty() {
            return;
        } else if self.scopes[s - 1].contains_key(&name.lexeme.clone()) {
            panic!("@error {} is already declared", name.clone().lexeme);
        }
        self.scopes[s - 1].insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            let s = self.scopes.len();
            self.scopes[s - 1].insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token, id: usize) {
        let s = self.scopes.len();
        if s != 0 {
            for i in (0..=(s - 1)).rev() {
                let scope = &self.scopes[i];
                if scope.contains_key(&name.lexeme) {
                    self.locals.insert(id, s - i - 1);
                    return;
                }
            }
        }
    }

    fn resolve_many(&mut self, stmts: &Vec<&Statement>, env: &mut Env) {
        for stmt in stmts {
            self.resolve_int(stmt, env)
        }
    }

    fn scope_start(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn scope_end(&mut self) {
        self.scopes.pop().expect("@error stack underflow");
    }
}
