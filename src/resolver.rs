// Absurd resolver, it resolves statements and returns locals
use crate::ast::{Statement, Token};
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
            Statement::For {
                iterator,
                index,
                body,
                expr,
            } => self.fors(iterator, index, body, expr, env),
            Statement::If {
                body,
                else_branch,
                cond,
            } => self.ifs(cond, body, else_branch, env),
            Statement::Block { stmts } => self.block(stmts, env),
            Statement::Break {} => self.breaks(),
            Statement::Expression { expr } => self.expr(expr, env),
            Statement::Func { body, params, .. } => {
                self.func(*body.clone(), params.as_slice(), env)
            }
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

    fn fors(
        &mut self,
        iterator: &Token,
        index: &Option<Token>,
        body: &Statement,
        expr: &Expression,
        env: &Rc<RefCell<Env>>,
    ) {
        self.expr(expr, env);
        let encl_loop = self.is_crnt_loop;
        self.is_crnt_loop = true;
        self.scope_start();
        self.declare(iterator);
        self.define(iterator);
        if let Some(i) = index {
            self.declare(i);
            self.define(i);
        }
        self.resolve_stmt(body, env);
        self.scope_end();
        self.is_crnt_loop = encl_loop;
    }

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

    fn whiles(&mut self, body: &Statement, cond: &Expression, env: &Rc<RefCell<Env>>) {
        let encl_loop = self.is_crnt_loop;
        self.expr(cond, env);
        self.is_crnt_loop = true;
        self.scope_start();
        self.resolve_stmt(body, env);
        self.scope_end();
        self.is_crnt_loop = encl_loop;
    }

    fn breaks(&mut self) {
        if !self.is_crnt_loop {
            self.err.throw(E0x302, 0, (0, 0), vec![]);
        }
    }

    fn func(&mut self, body: Statement, params: &[(Token, Token)], env: &Rc<RefCell<Env>>) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = true;
        self.scope_start();
        params.iter().for_each(|(name, _)| {
            self.declare(name);
            self.define(name);
        });
        match body {
            Statement::Block { stmts } => {
                self.resolve_many(stmts.as_slice(), env);
                stmts.iter().for_each(|stmt| {
                    if let Statement::Return { expr } = stmt {
                        self.expr(expr, env);
                    }
                });
            }
            _ => self.resolve_stmt(&body, env),
        }
        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn matchs(
        &mut self,
        cond: &Expression,
        def_case: &Statement,
        cases: &Vec<(Expression, Statement)>,
        env: &Rc<RefCell<Env>>,
    ) {
        self.expr(cond, env);
        cases.iter().for_each(|(case, body)| {
            self.scope_start();
            self.expr(case, env);
            self.resolve_stmt(body, env);
            self.scope_end();
        });

        self.resolve_stmt(def_case, env);
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
        body: &Box<Statement>,
        else_branch: &Option<Box<Statement>>,
        env: &Rc<RefCell<Env>>,
    ) {
        self.expr(cond, env);
        self.scope_start();
        self.resolve_stmt(body, env);
        self.scope_end();
        if let Some(branch) = else_branch {
            self.scope_start();
            self.resolve_stmt(branch, env);
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
            Expression::Record { fields, .. } => {
                fields.iter().for_each(|(_, val)| {
                    self.expr(val, env);
                });
            }
            Expression::If {
                cond,
                body,
                else_branch,
                ..
            } => {
                let cond = *(cond.clone());
                self.expr(&cond, env);
                let body = *(body.clone());
                self.expr(&body, env);
                if else_branch.is_some() {
                    let branch = *(else_branch.as_ref().unwrap().clone());
                    self.expr(&branch, env);
                }
            }
            Expression::Assign { value, .. } => self.expr(value, env),
            Expression::Vec { items, .. } => {
                items.iter().for_each(|item| self.expr(item, env));
            }
            Expression::Tuple { items, .. } => {
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
            Expression::Binary { left, right, .. } => {
                self.expr(left, env);
                self.expr(right, env);
            }
            Expression::Grouping { expression, .. } => self.expr(expression, env),
            _ => {}
        }
    }

    fn callback(&mut self, body: &Statement, params: &[(Token, Token)], env: &Rc<RefCell<Env>>) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = true;
        self.scope_start();
        params.iter().for_each(|(name, _)| {
            self.declare(name);
            self.define(name);
        });
        match body {
            Statement::Block { stmts } => {
                self.resolve_many(stmts.as_slice(), env);
                stmts.iter().for_each(|stmt| {
                    if let Statement::Return { expr } = stmt {
                        self.expr(expr, env);
                    }
                });
            }
            _ => self.resolve_stmt(&body, env),
        }

        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn varexpr(&mut self, expr: &Expression) {
        if let Expression::Var { name, .. } = expr {
            if let Some(false) = self.scopes.last().and_then(|scope| scope.get(&name.lexeme)) {
                self.err.throw(
                    E0x304,
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
                    .throw(E0x304, 0, (0, 0), vec!["a variable".to_string()]);
            }
        } else {
            self.err
                .throw(E0x304, 0, (0, 0), vec!["a variable".to_string()]);
        }
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                self.err
                    .throw(E0x305, name.line, name.pos, vec![name.lexeme.clone()]);
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
            self.err.throw(E0x306, 0, (0, 0), vec![]);
        }
    }
}
