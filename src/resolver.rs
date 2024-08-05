use crate::ast::{Statement, Token};
use crate::env::Env;
use crate::expr::Expression;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Bool {
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Resolver {
    locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    is_crnt_fnc: Bool,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            locals: HashMap::new(),
            scopes: Vec::new(),
            is_crnt_fnc: Bool::False,
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
            Statement::If { .. } => {}
            Statement::Block { .. } => {}
            Statement::Break { .. } => {}
            Statement::Enum { .. } => {}
            Statement::Expression { .. } => {}
            Statement::Func { .. } => {}
            Statement::Impl { .. } => {}
            Statement::Loop { .. } => {}
            Statement::Match { .. } => {}
            Statement::Mod { .. } => {}
            Statement::Return { .. } => {}
            Statement::Struct { .. } => {}
            Statement::Use { .. } => {}
            Statement::Var { .. } => {}
            Statement::While { .. } => {}
        }
    }

    fn resolve_block(&mut self, stmt: &Statement, env: &mut Env) {
        match stmt {
            Statement::Block { stmts } => {
                self.scope_start();
                self.resolve_many(&stmts.iter().collect(), env);
            }
            _ => panic!("@error failed to resolve a block statement"),
        }
    }

    fn resolve_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Array { items, .. } => {
                for item in items {
                    // resolve LiteralType
                }
            }
            Expression::Var { .. } => self.resolve_var_expr(expr),
            Expression::Call {
                name,
                args,
                call_type,
                ..
            } => {}
            Expression::Func {
                name,
                value_type,
                body,
                params,
                is_async,
                is_pub,
                ..
            } => {}
            Expression::Await { expr, .. } => self.resolve_expr(expr),
            Expression::Unary { left, .. } => self.resolve_expr(left),
            Expression::Value { value, .. } => {}
            Expression::Binary { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expression::Grouping { expression, .. } => self.resolve_expr(expression),
        }
    }

    fn resolve_var_expr(&mut self, expr: &Expression) {
        // let id = expr.clone().id();
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
