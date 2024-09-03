// Absurd resolver, it resolves statements and returns locals
use crate::ast::{FuncBody, Statement, Token};
use crate::errors::{Error, ErrorCode::*};
use crate::interpreter::env::Env;
use crate::interpreter::expr::Expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Resolver {
    locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    is_crnt_fnc: bool,
    is_crnt_loop: bool,
    err: Error,
}

impl Resolver {
    pub fn new(err: Error) -> Self {
        Resolver {
            locals: HashMap::new(),
            scopes: Vec::new(),
            is_crnt_fnc: false,
            is_crnt_loop: false,
            err,
        }
    }

    /// entry method
    pub fn resolve(
        &mut self,
        stmts: &[Statement],
        env: &Rc<RefCell<Env>>,
    ) -> HashMap<usize, usize> {
        stmts.iter().for_each(|stmt| self.resolve_stmt(stmt, env));
        self.locals.clone()
    }

    /// statement resolver
    fn resolve_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        match stmt {
            Statement::If {
                body,
                else_branch,
                else_if_branches,
                cond,
            } => self.ifs(cond, body, else_if_branches, else_branch, env),
            Statement::Block { stmts } => self.block(stmts, env),
            Statement::Break {} => self.breaks(),
            Statement::Enum { name, .. } => self.enums(name),
            Statement::Expression { expr } => self.expr(expr, env),
            Statement::Func { body, params, .. } => self.func(body, params.as_slice(), env),
            Statement::Loop { body, .. } => self.loops(body, env),
            Statement::Match {
                cond,
                cases,
                def_case,
            } => self.matchs(cond, def_case, cases, env),
            Statement::Return { expr } => self.returns(expr, env),
            Statement::Use { names, .. } => self.uses(names),
            Statement::Var { names, value, .. } => self.var(names, value, env),
            Statement::While { body, cond } => self.whiles(body, cond, env),
            _ => {}
        }
    }

    // everything below is self explanatory
    fn uses(&mut self, names: &Vec<(Token, Option<Token>)>) {
        for (old, new) in names {
            if let Some(new_name) = new {
                self.declare(new_name);
                self.define(new_name);
            } else {
                self.declare(old);
                self.define(old);
            }
        }
    }

    fn var(&mut self, names: &Vec<Token>, value: &Option<Expression>, env: &Rc<RefCell<Env>>) {
        for name in names {
            self.declare(name);
            if let Some(value) = value {
                self.expr(value, env);
            }
            self.define(name);
        }
    }

    fn whiles(&mut self, body: &Vec<Statement>, cond: &Expression, env: &Rc<RefCell<Env>>) {
        let encl_loop = self.is_crnt_loop;
        self.expr(cond, env);
        self.is_crnt_loop = true;
        self.resolve_many(body, env);
        self.is_crnt_loop = encl_loop;
    }

    fn breaks(&mut self) {
        if !self.is_crnt_loop {
            self.err.throw(E0x302, 0, (0, 0), vec![]);
        }
    }

    fn enums(&mut self, name: &Token) {
        self.declare(name);
        self.define(name);
    }

    fn func(&mut self, body: &FuncBody, params: &[(Token, Token)], env: &Rc<RefCell<Env>>) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = true;
        self.scope_start();
        params.iter().for_each(|(name, _)| {
            self.declare(name);
            self.define(name);
        });
        match body {
            FuncBody::Statements(body) => {
                self.resolve_many(body, env);
                body.iter().for_each(|stmt| {
                    if let Statement::Return { expr } = stmt {
                        self.expr(expr, env);
                    }
                });
            }
            FuncBody::Expression(expr) => self.expr(expr, env),
        }
        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn loops(&mut self, body: &Vec<Statement>, env: &Rc<RefCell<Env>>) {
        self.scope_start();
        let encl_loop = self.is_crnt_loop;
        self.is_crnt_loop = true;
        self.resolve_many(body, env);
        self.is_crnt_loop = encl_loop;
        self.scope_end();
    }

    fn matchs(
        &mut self,
        cond: &Expression,
        def_case: &FuncBody,
        cases: &Vec<(Expression, FuncBody)>,
        env: &Rc<RefCell<Env>>,
    ) {
        self.expr(cond, env);
        cases.iter().for_each(|(case, body)| {
            self.scope_start();
            self.expr(case, env);
            match body {
                FuncBody::Statements(stmts) => {
                    self.resolve_many(&stmts, env);
                }
                FuncBody::Expression(expr) => {
                    self.expr(expr, env);
                }
            }
            self.scope_end();
        });

        match def_case {
            FuncBody::Statements(stmts) => {
                if !stmts.is_empty() {
                    self.scope_start();
                    self.resolve_many(stmts, env);
                    self.scope_end();
                }
            }
            FuncBody::Expression(expr) => {
                self.expr(expr, env);
            }
        }
    }

    fn returns(&mut self, expr: &Expression, env: &Rc<RefCell<Env>>) {
        if self.is_crnt_fnc {
            self.expr(expr, env);
        } else {
            self.err.throw(E0x303, 0, (0, 0), vec![]);
        }
    }

    fn ifs(
        &mut self,
        cond: &Expression,
        body: &Vec<Statement>,
        else_if_branches: &Vec<(Expression, Vec<Statement>)>,
        else_branch: &Option<Vec<Statement>>,
        env: &Rc<RefCell<Env>>,
    ) {
        self.expr(cond, env);
        self.scope_start();
        self.resolve_many(body, env);
        self.scope_end();
        else_if_branches.iter().for_each(|(elif_pred, elif_stmt)| {
            self.expr(elif_pred, env);
            self.scope_start();
            self.resolve_many(elif_stmt, env);
            self.scope_end();
        });
        if let Some(branch) = else_branch {
            self.scope_start();
            self.resolve_many(branch, env);
            self.scope_end();
        }
    }

    fn block(&mut self, stmts: &Vec<Statement>, env: &Rc<RefCell<Env>>) {
        self.scope_start();
        self.resolve_many(stmts, env);
        self.scope_end();
    }

    fn expr(&mut self, expr: &Expression, env: &Rc<RefCell<Env>>) {
        match expr {
            Expression::Object { fields, .. } => {
                fields.iter().for_each(|(_, val)| {
                    self.expr(val, env);
                });
            }
            Expression::Method { args, .. } => args.iter().for_each(|arg| self.expr(arg, env)),
            Expression::Assign { value, .. } => self.expr(value, env),
            Expression::Array { items, .. } => {
                items.iter().for_each(|item| self.expr(item, env));
            }
            Expression::Var { .. } => self.varexpr(expr),
            Expression::Call { name, args, .. } => {
                self.expr(name.as_ref(), env);
                args.iter().for_each(|arg| self.expr(arg, env));
            }
            Expression::Func { body, params, .. } => self.callback(body, params, env),
            Expression::Await { expr, .. } => self.expr(expr, env),
            Expression::Unary { left, .. } => self.expr(left, env),
            Expression::Value { .. } => {}
            Expression::Binary { left, right, .. } => {
                self.expr(left, env);
                self.expr(right, env);
            }
            Expression::Grouping { expression, .. } => self.expr(expression, env),
        }
    }

    fn callback(&mut self, body: &FuncBody, params: &[(Token, Token)], env: &Rc<RefCell<Env>>) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = true;
        self.scope_start();
        params.iter().for_each(|(name, _)| {
            self.declare(name);
            self.define(name);
        });
        match body {
            FuncBody::Statements(body) => {
                self.resolve_many(body, env);
                body.iter().for_each(|stmt| {
                    if let Statement::Return { expr } = stmt {
                        self.expr(expr, env);
                    }
                });
            }
            FuncBody::Expression(expr) => {
                self.expr(expr, env);
                self.expr(expr, env);
            }
        }

        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn varexpr(&mut self, expr: &Expression) {
        if let Expression::Var { name, .. } = expr {
            if let Some(false) = self.scopes.last().and_then(|scope| scope.get(&name.lexeme)) {
                self.err.throw(
                    E0x306,
                    name.line,
                    name.pos,
                    vec!["a local variable".to_string()],
                );
            }
        } else if let Expression::Call { name, .. } = expr {
            if let Expression::Var { name, .. } = name.as_ref() {
                self.resolve_local(name, expr.id());
            } else {
                self.err
                    .throw(E0x306, 0, (0, 0), vec!["a variable".to_string()]);
            }
        } else {
            self.err
                .throw(E0x306, 0, (0, 0), vec!["a variable".to_string()]);
        }
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                self.err
                    .throw(E0x307, name.line, name.pos, vec![name.lexeme.clone()]);
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token, id: usize) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(id, i);
                return;
            }
        }
    }

    fn resolve_many(&mut self, stmts: &[Statement], env: &Rc<RefCell<Env>>) {
        stmts.iter().for_each(|stmt| self.resolve_stmt(stmt, env));
    }

    fn scope_start(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn scope_end(&mut self) {
        if self.scopes.pop().is_none() {
            self.err.throw(E0x308, 0, (0, 0), vec![]);
        }
    }
}
