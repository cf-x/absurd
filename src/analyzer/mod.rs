mod utils;
use crate::ast::{
    FuncBody,
    Statement::{self, *},
    Token, TokenType,
};
use crate::expr::Expression;

#[derive(Debug, Clone)]
pub struct FuncCall {
    name: Token,
    args: Vec<Expression>,
}

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
            Use { src, names } => self.uses(src, names),
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

    fn expr(&mut self, expr: Expression) -> Option<Statement> {
        None
    }
    fn block(&mut self, vec: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn var(
        &mut self,
        names: Vec<Token>,
        value_type: Token,
        value: Option<Expression>,
        is_mut: bool,
        is_pub: bool,
        is_func: bool,
        pub_namees: Vec<Token>,
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
        name: Token,
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
        is_impl: bool,
        is_mut: bool,
    ) -> Option<Statement> {
        None
    }

    fn ifs(
        &mut self,
        cond: Expression,
        body: Vec<Statement>,
        else_if_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    ) -> Option<Statement> {
        None
    }

    fn returns(&mut self, expr: Expression) -> Option<Statement> {
        None
    }

    fn whiles(&mut self, cond: Expression, body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn loops(&mut self, iter: Option<usize>, body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn breaks(&mut self) -> Option<Statement> {
        None
    }

    fn matchs(
        &mut self,
        cond: Expression,
        cases: Vec<(Expression, FuncBody)>,
        def_case: FuncBody,
    ) -> Option<Statement> {
        None
    }

    fn mods(&mut self, src: String) -> Option<Statement> {
        None
    }

    fn uses(&mut self, src: String, names: Vec<(Token, Option<Token>)>) -> Option<Statement> {
        None
    }

    fn structs(
        &mut self,
        name: Token,
        structs: Vec<(Token, TokenType, bool)>,
        is_pub: bool,
        methods: Vec<(Expression, bool)>,
    ) -> Option<Statement> {
        None
    }

    fn impls(&mut self, name: Token, body: Vec<Statement>) -> Option<Statement> {
        None
    }

    fn enums(&mut self, name: Token, enums: Vec<Token>, is_pub: bool) -> Option<Statement> {
        None
    }
}
