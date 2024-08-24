mod meta;
use crate::ast::{
    FuncBody,
    Statement::{self, *},
    Token, TokenType,
};
use crate::interpreter::expr::Expression;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuncCall {
    name: Token,
    args: Vec<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Called {
    Var(Token),
    Func(FuncCall),
}

#[derive(Debug, Clone)]
pub struct Analyzer {
    input_ast: Vec<Statement>,
    called: Vec<Called>,
    crnt: usize,
}

impl Analyzer {
    pub fn new(ast: Vec<Statement>) -> Self {
        Analyzer {
            input_ast: ast,
            called: vec![],
            crnt: 0,
        }
    }

    pub fn analyze(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];
        let mut rev_ast = self.input_ast.clone();
        rev_ast.reverse();
        for stmt in rev_ast {
            let s = self.statement(stmt);
            if s.is_some() {
                stmts.push(s.unwrap());
            }
            self.crnt += 1;
        }
        stmts
    }

    pub fn statement(&mut self, stmt: Statement) -> Option<Statement> {
        match stmt {
            Statement::Expression { expr } => self.expr(expr),
            Block { stmts } => self.block(stmts),
            Var {
                names,
                value_type,
                value,
                is_mut,
                is_pub,
                pub_names,
                is_func,
            } => self.var(names, value_type, value, is_mut, is_pub, is_func, pub_names),
            Func {
                name,
                value_type,
                body,
                params,
                is_async,
                is_pub,
                is_impl,
                is_mut,
            } => self.func(
                name, value_type, body, params, is_async, is_pub, is_impl, is_mut,
            ),
            If {
                cond,
                body,
                else_if_branches,
                else_branch,
            } => self.ifs(cond, body, else_if_branches, else_branch),
            Return { expr } => self.returns(expr),
            While { cond, body } => self.whiles(cond, body),
            Loop { iter, body } => self.loops(iter, body),
            Break {} => self.breaks(),
            Match {
                cond,
                cases,
                def_case,
            } => self.matchs(cond, cases, def_case),
            Mod { src } => self.mods(src),
            Use { src, names, all } => self.uses(src, names, all),
            Struct {
                name,
                structs,
                is_pub,
                methods,
            } => self.structs(name, structs, is_pub, methods),
            Impl { name, body } => self.impls(name, body),
            Enum {
                name,
                enums,
                is_pub,
            } => self.enums(name, enums, is_pub),
        }
    }

    fn expr(&mut self, _expr: Expression) -> Option<Statement> {
        None
    }
    fn block(&mut self, _vec: Vec<Statement>) -> Option<Statement> {
        None
    }
    fn var(
        &mut self,
        names: Vec<Token>,
        _value_type: Token,
        _value: Option<Expression>,
        _is_mut: bool,
        is_pub: bool,
        _is_func: bool,
        _pub_namees: Vec<Token>,
    ) -> Option<Statement> {
        if !is_pub {
            for name in names {
                if !self.is_called(Called::Var(name.clone()), name.clone()) {
                    eprintln!("warning: unused variable '{}'", name.lexeme);
                }
            }
        }
        None
    }

    fn func(
        &mut self,
        _name: Token,
        _value_type: Token,
        _body: FuncBody,
        _params: Vec<(Token, Token)>,
        _is_async: bool,
        _is_pub: bool,
        _is_impl: bool,
        _is_mut: bool,
    ) -> Option<Statement> {
        None
    }

    fn ifs(
        &mut self,
        _cond: Expression,
        _body: Vec<Statement>,
        _else_if_branches: Vec<(Expression, Vec<Statement>)>,
        _else_branch: Option<Vec<Statement>>,
    ) -> Option<Statement> {
        None
    }

    fn returns(&mut self, _expr: Expression) -> Option<Statement> {
        None
    }

    fn whiles(&mut self, _cond: Expression, _body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn loops(&mut self, _iter: Option<usize>, _body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn breaks(&mut self) -> Option<Statement> {
        None
    }

    fn matchs(
        &mut self,
        _cond: Expression,
        _cases: Vec<(Expression, FuncBody)>,
        _def_case: FuncBody,
    ) -> Option<Statement> {
        None
    }

    fn mods(&mut self, _src: String) -> Option<Statement> {
        None
    }

    fn uses(
        &mut self,
        _src: String,
        _names: Vec<(Token, Option<Token>)>,
        _all: bool,
    ) -> Option<Statement> {
        None
    }

    fn structs(
        &mut self,
        _name: Token,
        _structs: Vec<(Token, TokenType, bool)>,
        _is_pub: bool,
        _methods: Vec<(Expression, bool)>,
    ) -> Option<Statement> {
        None
    }

    fn impls(&mut self, _name: Token, _body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn enums(&mut self, _name: Token, _enums: Vec<Token>, _is_pub: bool) -> Option<Statement> {
        None
    }
}

#[allow(dead_code)]
impl Analyzer {
    pub fn next(&mut self) -> Statement {
        let mut ast = self.input_ast.clone();
        ast.reverse();
        if !self.is_eof() {
            return ast.get(self.crnt + 1).unwrap().clone();
        }
        ast.get(self.crnt).unwrap().clone()
    }
    pub fn is_eof(&mut self) -> bool {
        if self.input_ast.clone().len() <= self.crnt {
            return true;
        }
        false
    }

    pub fn is_called(&self, kind: Called, name: Token) -> bool {
        match kind {
            Called::Var(_) => self
                .called
                .iter()
                .any(|c| matches!(c, Called::Var(n) if *n == name)),
            Called::Func(_) => self
                .called
                .iter()
                .any(|c| matches!(c, Called::Func(n) if n.name == name)),
        }
    }

    pub fn push_called(&mut self, callee: Called) {
        self.called.push(callee);
    }
}
