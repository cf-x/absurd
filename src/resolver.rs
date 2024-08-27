use crate::ast::{FuncBody, LiteralKind, LiteralType, Statement, Token, TokenType};
use crate::interpreter::env::Env;
use crate::interpreter::expr::Expression;
use crate::interpreter::types::TypeKind;
use crate::utils::errors::{Error, ErrorCode::*};
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

    pub fn resolve(
        &mut self,
        stmts: &[Statement],
        env: &Rc<RefCell<Env>>,
    ) -> HashMap<usize, usize> {
        self.resolve_stmts(stmts, env);
        self.locals.clone()
    }

    fn resolve_stmts(&mut self, stmts: &[Statement], env: &Rc<RefCell<Env>>) {
        for stmt in stmts {
            self.resolve_int(stmt, env);
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
            Statement::Loop { .. } => self.resolve_loop_stmt(stmt, env),
            Statement::Match { .. } => self.resolve_match_stmt(stmt, env),
            Statement::Mod { .. } => {}
            Statement::Return { .. } => self.resolve_return_stmt(stmt, env),
            Statement::Use { .. } => self.resolve_use_stmt(stmt),
            Statement::Var { .. } => self.resolve_var_stmt(stmt, env),
            Statement::While { .. } => self.resolve_while_stmt(stmt, env),
        }
    }

    fn resolve_use_stmt(&mut self, stmt: &Statement) {
        if let Statement::Use { names, .. } = stmt {
            for name in names {
                let (old, new) = name;
                if let Some(new_name) = new {
                    self.declare(new_name);
                    self.define(new_name);
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
        }
    }

    fn resolve_while_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::While { body, cond } = stmt {
            let encl_loop = self.is_crnt_loop;
            self.resolve_expr(cond, env);
            self.is_crnt_loop = true;
            self.resolve_many(body, env);
            self.is_crnt_loop = encl_loop;
        }
    }

    fn resolve_break_stmt(&mut self) {
        if !self.is_crnt_loop {
            self.err.throw(E0x302, 0, (0, 0), vec![]);
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
            self.is_crnt_fnc = true;
            self.scope_start();
            for (parname, _) in params {
                self.declare(parname);
                self.define(parname);
            }

            if let FuncBody::Statements(body) = body {
                self.resolve_many(body, env);

                for stmt in body {
                    if let Statement::Return { expr } = stmt {
                        let val = (*expr).eval(Rc::clone(&env));
                        if type_check(value_type, &val) {
                            self.resolve_expr(expr, env);
                        } else if params.is_empty() {
                            self.err.throw(
                                E0x301,
                                name.line,
                                name.pos,
                                vec![value_type.clone().lexeme, val.to_string()],
                            );
                        }
                    }
                }
            } else {
                self.err.throw(E0x305, 0, (0, 0), vec![]);
            }

            self.scope_end();
            self.is_crnt_fnc = encl_func;
        }
    }

    fn resolve_loop_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Loop { body, .. } = stmt {
            self.scope_start();
            let encl_loop = self.is_crnt_loop;
            self.is_crnt_loop = true;
            self.resolve_many(body, env);
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
                        self.resolve_many(stmts, env);
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
                        self.resolve_many(stmts, env);
                        self.scope_end();
                    }
                }
                FuncBody::Expression(expr) => {
                    self.resolve_expr(expr, env);
                }
            }
        }
    }

    fn resolve_return_stmt(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if self.is_crnt_fnc {
            if let Statement::Return { expr } = stmt {
                self.resolve_expr(expr, env);
            }
        } else {
            self.err.throw(E0x303, 0, (0, 0), vec![]);
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
            self.resolve_many(body, env);
            self.scope_end();
            for (elif_pred, elif_stmt) in else_if_branches {
                self.resolve_expr(elif_pred, env);
                self.scope_start();
                self.resolve_many(elif_stmt, env);
                self.scope_end();
            }
            if let Some(branch) = else_branch {
                self.scope_start();
                self.resolve_many(branch, env);
                self.scope_end();
            }
        }
    }

    fn resolve_block(&mut self, stmt: &Statement, env: &Rc<RefCell<Env>>) {
        if let Statement::Block { stmts } = stmt {
            self.scope_start();
            self.resolve_many(stmts, env);
            self.scope_end();
        } else {
            self.err
                .throw(E0x306, 0, (0, 0), vec!["a block statement".to_string()]);
        }
    }

    fn resolve_expr(&mut self, expr: &Expression, env: &Rc<RefCell<Env>>) {
        match expr {
            Expression::Method { args, .. } => {
                for arg in args {
                    self.resolve_expr(arg, env);
                }
            }
            Expression::Assign { value, .. } => {
                self.resolve_expr(value, env);
            }
            Expression::Array { items, .. } => {
                for item in items {
                    self.resolve_expr(item, env);
                }
            }
            Expression::Var { .. } => self.resolve_var_expr(expr),
            Expression::Call { name, args, .. } => {
                self.resolve_expr(name.as_ref(), env);
                for arg in args {
                    self.resolve_expr(arg, env);
                }
            }
            Expression::Func {
                value_type,
                body,
                params,
                is_async,
                is_pub,
                ..
            } => self.resolve_func_expr(value_type, body, params, *is_async, *is_pub, env),
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
        params: &[(Token, Token)],
        _is_async: bool,
        _is_pub: bool,
        env: &Rc<RefCell<Env>>,
    ) {
        let encl_func = self.is_crnt_fnc;
        self.is_crnt_fnc = true;
        self.scope_start();
        for (parname, _) in params {
            self.declare(parname);
            self.define(parname);
        }

        if let FuncBody::Statements(body) = body {
            self.resolve_many(body, env);

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
        } else {
            self.err.throw(E0x305, 0, (0, 0), vec![]);
        }

        self.scope_end();
        self.is_crnt_fnc = encl_func;
    }

    fn resolve_var_expr(&mut self, expr: &Expression) {
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
        for stmt in stmts {
            self.resolve_int(stmt, env);
        }
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

pub fn type_check(value_type: &Token, val: &LiteralType) -> bool {
    match value_type.token {
        TokenType::NumberIdent => matches!(val, LiteralType::Number(_)),
        TokenType::StringIdent => matches!(val, LiteralType::String(_)),
        TokenType::BoolIdent => matches!(val, LiteralType::Boolean(_)),
        TokenType::CharIdent => matches!(val, LiteralType::Char(_)),
        TokenType::NullIdent => matches!(val, LiteralType::Null),
        TokenType::VoidIdent => matches!(val, LiteralType::Void),
        TokenType::ArrayIdent => {
            if let LiteralType::Array(array) = val {
                match value_type.clone().value {
                    Some(value) => match value {
                        LiteralKind::Type(t) => {
                            let t = *t;
                            match t {
                                TypeKind::Array { statics, .. } => {
                                    if let Some(statics) = statics {
                                        if statics.len() != array.len() {
                                            return false;
                                        }

                                        return statics.iter().zip(array.iter()).all(
                                            |(stat, item)| {
                                                let stat_token = match stat.clone() {
                                                    TypeKind::Var { name } => name,
                                                    _ => value_type.clone(),
                                                };
                                                type_check(
                                                    &Token {
                                                        token: stat_token.token,
                                                        lexeme: stat_token.lexeme.clone(),
                                                        value: None,
                                                        line: stat_token.line,
                                                        pos: stat_token.pos,
                                                    },
                                                    &item.to_literal(),
                                                )
                                            },
                                        );
                                    }

                                    return array.iter().all(|item| {
                                        type_check(
                                            &Token {
                                                token: string_to_token_type(&value_type.lexeme),
                                                lexeme: value_type.lexeme.clone(),
                                                value: None,
                                                line: value_type.line,
                                                pos: value_type.pos,
                                            },
                                            &item.to_literal(),
                                        )
                                    });
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    None => {}
                }

                false
            } else {
                false
            }
        }
        TokenType::AnyIdent => true,
        TokenType::NumberLit
        | TokenType::StringLit
        | TokenType::TrueLit
        | TokenType::FalseLit
        | TokenType::CharLit
        | TokenType::NullLit
        | TokenType::ArrayLit => {
            match val.clone() {
                LiteralType::Number(num) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap()), LiteralType::Number(n) if n == num);
                }
                LiteralType::String(s) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap()), LiteralType::String(n) if n == s);
                }
                LiteralType::Boolean(b) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap()), LiteralType::Boolean(n) if n == b);
                }
                LiteralType::Char(c) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap()), LiteralType::Char(n) if n == c);
                }
                LiteralType::Array(v) => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(literalkind_to_literaltype(value_type.clone().value.unwrap()), LiteralType::Array(n) if n == v);
                }
                LiteralType::Null => {
                    return matches!(value_type.token, TokenType::NumberLit)
                        && matches!(
                            literalkind_to_literaltype(value_type.clone().value.unwrap()),
                            LiteralType::Null
                        );
                }
                _ => {}
            }
            false
        }
        _ => false,
    }
}

fn literalkind_to_literaltype(kind: LiteralKind) -> LiteralType {
    match kind {
        LiteralKind::Bool { value } => LiteralType::Boolean(value),
        LiteralKind::Null => LiteralType::Null,
        LiteralKind::Char { value } => LiteralType::Char(value),
        LiteralKind::Number { value, .. } => LiteralType::Number(value),
        LiteralKind::String { value } => LiteralType::String(value),
        LiteralKind::Type(t) => typekind_to_literaltype(*t),
    }
}

fn typekind_to_literaltype(kind: TypeKind) -> LiteralType {
    match kind {
        TypeKind::Var { name } => literalkind_to_literaltype(name.value.unwrap()),
        TypeKind::Func { ret, .. } => typekind_to_literaltype(*ret),
        TypeKind::Array { kind, statics } => {
            if kind.is_some() {
                typekind_to_literaltype(*kind.unwrap())
            } else {
                typekind_to_literaltype(statics.unwrap().get(0).unwrap().clone())
            }
        }
        TypeKind::Value { kind } => literalkind_to_literaltype(kind),
    }
}

fn string_to_token_type(s: &str) -> TokenType {
    match s {
        "number" => TokenType::NumberIdent,
        "string" => TokenType::StringIdent,
        "boolean" => TokenType::BoolIdent,
        "char" => TokenType::CharIdent,
        "null" => TokenType::NullIdent,
        "void" => TokenType::VoidIdent,
        _ => TokenType::AnyIdent,
    }
}
