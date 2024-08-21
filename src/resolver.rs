use crate::ast::{FuncBody, LiteralType, Statement, Token, TokenType};
use crate::env::Env;
use crate::errors::{Error, ErrorCode::*};
use crate::expr::Expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    err: Error,
}

impl Resolver {
    pub fn new(src: &str) -> Self {
        Resolver {
            locals: HashMap::new(),
            scopes: Vec::new(),
            is_crnt_fnc: Bool::False,
            is_crnt_loop: Bool::False,
            err: Error::new(src),
        }
    }
    pub fn resolve(
        &mut self,
        stmts: &Vec<&Statement>,
        env: &Rc<RefCell<Env>>,
    ) -> HashMap<usize, usize> {
        self.resolve_stmts(stmts, env);
        self.locals.clone()
    }

    fn resolve_stmts(&mut self, stmts: &Vec<&Statement>, env: &Rc<RefCell<Env>>) {
        for stmt in stmts {
            self.resolve_int(stmt, env)
        }
    }

    fn resolve_int(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        match stmt {
            Statement::If { .. } => self.resolve_if_stmt(stmt, env),
            Statement::Block { .. } => self.resolve_block(stmt, env),
            Statement::Break {} => self.resolve_break_stmt(),
            Statement::Enum { .. } => self.resolve_enum_stmt(stmt),
            Statement::Expression { expr } => self.resolve_expr(expr, env),
            Statement::Func { .. } => self.resolve_func_stmt(stmt, env),
            Statement::Impl { .. } => self.resolve_impl_stmt(stmt, env),
            Statement::Loop { .. } => self.resolve_loop_stmt(stmt, env),
            Statement::Match { .. } => self.resolve_match_stmt(stmt, env),
            Statement::Mod { .. } => self.resolve_mod_stmt(stmt, env),
            Statement::Return { .. } => self.resolve_return_stmt(stmt, env),
            Statement::Struct { .. } => self.resolve_struct_stmt(stmt, env),
            Statement::Use { .. } => self.resolve_use_stmt(stmt),
            Statement::Var { .. } => self.resolve_var_stmt(stmt, env),
            Statement::While { .. } => self.resolve_while_stmt(stmt, env),
        }
    }

    fn resolve_struct_stmt(&mut self, stmt: &Statement, _env: &Rc<RefCell<Env>>) {
        if let Statement::Struct { .. } = stmt {
            // @todo
        }
    }

    fn resolve_use_stmt(&mut self, stmt: &Statement) {
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

    fn resolve_var_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Var {
            names,
            pub_names,
            value,
            value_type,
            ..
        } = stmt
        {
            for name in names {
                self.declare(name);
                if let Some(value) = value {
                    let val = match value {
                        Expression::Call { .. } => LiteralType::Any,
                        _ => value.eval(Rc::clone(&env)),
                    };
                    if type_check(value_type, &val) {
                        self.resolve_expr(value, env);
                    } else {
                        self.err.throw(
                            E0x301,
                            name.line,
                            name.pos,
                            vec![value_type.clone().lexeme, val.to_string()],
                        );
                    }
                }
                self.define(name);
            }
            // @todo better handle public names (after adding modality)
            for pub_name in pub_names {
                self.declare(pub_name);
                if let Some(value) = value {
                    self.resolve_expr(value, env);
                }
                self.define(pub_name);
            }
        }
    }

    fn resolve_while_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::While { body, cond } = stmt {
            let encl_loop = self.is_crnt_loop;
            self.resolve_expr(cond, env);
            self.is_crnt_loop = Bool::True;
            self.resolve_many(&body.iter().collect(), env);
            self.is_crnt_loop = encl_loop;
        }
    }

    fn resolve_break_stmt(&mut self) {
        match self.is_crnt_loop {
            Bool::True => {}
            Bool::False => {
                self.err.throw(E0x302, 0, (0, 0), vec![]);
            }
        }
    }

    fn resolve_enum_stmt(&mut self, stmt: &Statement) {
        if let Statement::Enum { name, .. } = stmt {
            self.declare(name);
            self.define(name);
        }
    }

    fn resolve_func_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Func {
            value_type,
            body,
            params,
            name,
            ..
        } = stmt
        {
            let encl_func = self.is_crnt_fnc;
            self.is_crnt_fnc = Bool::True;
            self.scope_start();
            for (parname, _) in params {
                self.declare(parname);
                self.define(parname);
            }

            match body {
                FuncBody::Statements(body) => {
                    self.resolve_many(&body.iter().collect(), env);

                    for stmt in body {
                        if let Statement::Return { expr } = stmt {
                            let val = (*expr).eval(Rc::clone(&env));
                            if type_check(value_type, &val) {
                                self.resolve_expr(expr, env);
                            } else if params.len() == 0 {
                                self.err.throw(
                                    E0x301,
                                    name.line,
                                    name.pos,
                                    vec![value_type.clone().lexeme, val.to_string()],
                                );
                            }
                        }
                    }
                }
                _ => {
                    self.err.throw(E0x305, 0, (0, 0), vec![]);
                }
            }

            self.scope_end();
            self.is_crnt_fnc = encl_func;
        }
    }

    fn resolve_impl_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Impl { body, .. } = stmt {
            self.scope_start();
            self.resolve_many(&body.iter().collect(), env);
            self.scope_end();
        }
    }

    fn resolve_loop_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Loop { body, .. } = stmt {
            self.scope_start();
            let encl_loop = self.is_crnt_loop;
            self.is_crnt_loop = Bool::True;
            self.resolve_many(&body.iter().collect(), env);
            self.is_crnt_loop = encl_loop;
            self.scope_end();
        }
    }

    fn resolve_match_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
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

    fn resolve_mod_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Mod { src } = stmt {
            env.borrow_mut()
                .define_mod(src.clone(), LiteralType::String(src.clone()));
        }
    }

    fn resolve_return_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        match self.is_crnt_fnc {
            Bool::True => {
                if let Statement::Return { expr } = stmt {
                    self.resolve_expr(expr, env);
                }
            }
            _ => {
                self.err.throw(E0x303, 0, (0, 0), vec![]);
            }
        }
    }

    fn resolve_if_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
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

    fn resolve_block(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        match stmt {
            Statement::Block { stmts } => {
                self.scope_start();
                self.resolve_many(&stmts.iter().collect(), env);
                self.scope_end();
            }
            _ => self
                .err
                .throw(E0x306, 0, (0, 0), vec!["a block statement".to_string()]),
        }
    }

    fn resolve_expr(&mut self, expr: &Expression, env: &Rc<RefCell<Env>>) {
        match expr {
            Expression::Array { items, .. } => {
                for item in items {
                    self.resolve_expr(item, env)
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
        _is_async: &bool,
        _is_pub: &bool,
        env: &Rc<RefCell<Env>>,
    ) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = Bool::True;
        self.scope_start();
        for (parname, _) in params {
            self.declare(parname);
            self.define(parname);
        }

        match body {
            FuncBody::Statements(body) => {
                self.resolve_many(&body.iter().collect(), env);

                for stmt in body {
                    if let Statement::Return { expr } = stmt {
                        let val = (*expr).eval(Rc::clone(&env));
                        if type_check(value_type, &val) {
                            self.resolve_expr(expr, env);
                        } else {
                            self.err.throw(
                                E0x301,
                                0,
                                (0, 0),
                                vec![value_type.clone().lexeme, val.to_string()],
                            );
                        }
                    }
                }
            }
            _ => {
                self.err.throw(E0x305, 0, (0, 0), vec![]);
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
                        self.err.throw(
                            E0x306,
                            name.line,
                            name.pos,
                            vec!["a local variable".to_string()],
                        )
                    }
                }
            }
            Expression::Call { name, .. } => match name.as_ref() {
                Expression::Var { name, .. } => self.resolve_local(&name, expr.clone().id()),
                _ => self
                    .err
                    .throw(E0x306, 0, (0, 0), vec!["a variable".to_string()]),
            },
            _ => self
                .err
                .throw(E0x306, 0, (0, 0), vec!["a variable".to_string()]),
        }
    }

    fn declare(&mut self, name: &Token) {
        let s = self.scopes.len();
        // @todo: add shadowing
        if self.scopes.is_empty() {
            return;
        } else if self.scopes[s - 1].contains_key(&name.lexeme.clone()) {
            self.err
                .throw(E0x307, name.line, name.pos, vec![name.clone().lexeme]);
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

    fn resolve_many(&mut self, stmts: &Vec<&Statement>, env: &Rc<RefCell<Env>>) {
        for stmt in stmts {
            self.resolve_int(stmt, env)
        }
    }

    fn scope_start(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn scope_end(&mut self) {
        match self.scopes.pop() {
            Some(_) => {}
            None => {
                self.err.throw(E0x308, 0, (0, 0), vec![]);
            }
        }
    }
}

pub fn type_check(value_type: &Token, val: &LiteralType) -> bool {
    match value_type.token {
        TokenType::NumberIdent => {
            if let LiteralType::Number(_) = val {
                true
            } else {
                false
            }
        }
        TokenType::StringIdent => {
            if let LiteralType::String(_) = val {
                true
            } else {
                false
            }
        }
        TokenType::BoolIdent => {
            if let LiteralType::Boolean(_) = val {
                true
            } else {
                false
            }
        }
        TokenType::CharIdent => {
            if let LiteralType::Char(_) = val {
                true
            } else {
                false
            }
        }
        TokenType::NullIdent => {
            if let LiteralType::Null = val {
                true
            } else {
                false
            }
        }
        TokenType::VoidIdent => {
            if let LiteralType::Void = val {
                true
            } else {
                false
            }
        }
        TokenType::AnyIdent => true,
        _ => false,
    }
}
