use crate::ast::{FuncBody, LiteralKind, LiteralType, Statement, Token, TokenType::*};
use crate::interpreter::expr::Expression;
use crate::utils::errors::{Error, ErrorCode::*};
pub mod expr;
mod helpers;
mod types;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    err: Error,
    crnt: usize,
    pub id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, err: Error) -> Self {
        Self {
            tokens,
            err,
            crnt: 0,
            id: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];
        while !self.check(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        stmts
    }

    fn stmt(&mut self) -> Statement {
        self.advance();
        match self.prev(1).token {
            Let => self.var_stmt(),
            Func => self.func_stmt(),
            If => self.if_stmt(),
            Return => self.return_stmt(),
            While => self.while_stmt(),
            Loop => self.loop_stmt(),
            Break => self.break_stmt(),
            Match => self.match_stmt(),
            Mod => self.mod_stmt(),
            Use => self.use_stmt(),
            Enum => self.enum_stmt(),
            LeftBrace => self.block_stmt(),
            TypeStmt => self.type_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn type_stmt(&mut self) -> Statement {
        let is_pub = self.if_token_consume(Pub);
        let name = self.consume(Ident);
        self.consume(Assign);
        let value = self.consume_type();
        self.consume(Semi);
        Statement::Type {
            name,
            value,
            is_pub,
        }
    }

    fn var_stmt(&mut self) -> Statement {
        let mut names = vec![];
        let mut pub_names = vec![];
        let is_mut = self.if_token_consume(Mut);
        let mut is_pub = false;
        let mut is_null = false;
        let mut is_arr_dest = false;

        if !is_mut && self.if_token_consume(Pub) {
            is_pub = true;
            if self.if_token_consume(LeftParen) {
                loop {
                    if self.if_token_consume(Underscore) {
                        pub_names.push(Token {
                            token: NullLit,
                            lexeme: "null".to_string(),
                            value: None,
                            line: self.peek().line,
                            pos: self.peek().pos,
                        })
                    } else {
                        let name = self.consume(Ident);
                        pub_names.push(name);
                    }
                    if !self.if_token_consume(Comma) || self.is_token(RightParen) {
                        break;
                    }
                }
                self.consume(RightParen);
            }
        }

        if self.if_token_consume(LeftBracket) {
            is_arr_dest = true;
            while !self.if_token_consume(RightBracket) {
                if self.if_token_consume(Underscore) {
                    names.push(Token {
                        token: NullLit,
                        lexeme: "null".to_string(),
                        value: None,
                        line: self.peek().line,
                        pos: self.peek().pos,
                    });
                } else {
                    let name = self.consume(Ident);
                    names.push(name);
                }
                if !self.is_token(Comma) || self.is_token(Colon) {
                    break;
                }
                self.advance();
            }
            self.consume(RightBracket);
        } else {
            loop {
                let name = self.consume(Ident);
                names.push(name);

                if self.is_token(Semi) {
                    is_null = true;
                    break;
                }

                if !self.is_token(Comma) || self.is_token(Colon) {
                    break;
                }
                self.advance();
            }
        }

        if pub_names.is_empty() {
            pub_names = names.clone();
        }

        let null_var = Statement::Var {
            names: names.clone(),
            value_type: self.create_null_token(names[0].line),
            value: Some(Expression::Value {
                id: self.id(),
                value: LiteralType::Null,
            }),
            is_mut,
            is_pub,
            pub_names: pub_names.clone(),
            is_func: false,
            is_arr_dest,
        };

        if is_null {
            self.advance();
            return null_var;
        }

        self.consume(Colon);
        let value_type = self.consume_type();
        if value_type.token == NullIdent {
            return null_var;
        }

        if self.if_token_consume(Semi) {
            return Statement::Var {
                names: names.clone(),
                value_type,
                value: Some(Expression::Value {
                    id: self.id(),
                    value: LiteralType::Null,
                }),
                is_mut,
                is_pub,
                pub_names: pub_names.clone(),
                is_func: false,
                is_arr_dest,
            };
        }

        self.consume(Assign);
        let is_func = self.is_token(Pipe);
        let value = self.expr();
        self.consume(Semi);

        Statement::Var {
            names,
            value_type,
            value: Some(value),
            is_mut,
            is_pub,
            pub_names,
            is_func,
            is_arr_dest,
        }
    }

    fn func_stmt(&mut self) -> Statement {
        let mut params = vec![];
        let mut is_async = false;
        let mut is_pub = false;

        if self.if_token_consume(Pub) {
            is_pub = true;
            if self.if_token_consume(Async) {
                is_async = true;
            }
        }

        if self.if_token_consume(Async) {
            is_async = true;
            if self.if_token_consume(Pub) {
                is_pub = true;
            }
        }

        let name = self.consume(Ident);

        self.consume(LeftParen);
        while !self.if_token_consume(RightParen) {
            if self.is_token(Ident) {
                let param_name = self.consume(Ident);
                self.consume(Colon);
                let param_type = self.consume_type();
                params.push((param_name, param_type))
            } else if self.if_token_consume(Comma) {
            } else if !self.is_token(RightParen) {
                self.throw_error(E0x201, vec![self.peek().lexeme]);
            }
        }
        self.consume(Arrow);
        let value_type = self.consume_type();

        if self.if_token_consume(Assign) {
            let body = self.expr();
            self.consume(Semi);
            return Statement::Func {
                name,
                value_type,
                body: FuncBody::Statements(vec![Statement::Return { expr: body }]),
                params,
                is_async,
                is_pub,
            };
        }

        self.consume(LeftBrace);
        let body = self.block_stmts();

        Statement::Func {
            name,
            value_type,
            body: FuncBody::Statements(body),
            params,
            is_async,
            is_pub,
        }
    }
    fn if_stmt(&mut self) -> Statement {
        let cond = self.expr();
        let body = self.block_stmts();
        let mut else_if_branches = vec![];

        while self.if_token_consume(ElseIf) {
            let elif_preds = self.expr();
            let elif_stmt = self.block_stmts();
            else_if_branches.push((elif_preds, elif_stmt))
        }

        let else_branch = if self.if_token_consume(Else) {
            Some(self.block_stmts())
        } else {
            None
        };

        Statement::If {
            cond,
            body,
            else_if_branches,
            else_branch,
        }
    }

    fn return_stmt(&mut self) -> Statement {
        let expr = if self.is_token(Semi) {
            Expression::Value {
                id: self.id(),
                value: LiteralType::Null,
            }
        } else {
            self.expr()
        };
        self.consume(Semi);
        Statement::Return { expr }
    }

    fn while_stmt(&mut self) -> Statement {
        let cond = self.expr();
        let body = self.block_stmts();
        Statement::While { cond, body }
    }

    fn loop_stmt(&mut self) -> Statement {
        let iter = if self.is_token(NumberLit) {
            let num = match self.consume(NumberLit).value {
                Some(LiteralKind::Number { value, .. }) => value,
                _ => self.throw_error(E0x202, vec![self.peek().lexeme]),
            };
            if num < 0.0 {
                Some(1)
            } else {
                Some(num as usize)
            }
        } else {
            None
        };
        let body = self.block_stmts();
        Statement::Loop { iter, body }
    }

    fn break_stmt(&mut self) -> Statement {
        self.consume(Semi);
        Statement::Break {}
    }

    fn match_stmt(&mut self) -> Statement {
        let cond = self.expr();
        self.consume(LeftBrace);
        let mut cases = vec![];

        while self.is_literal() || self.is_uppercase_ident() {
            let expr = self.expr();
            self.consume(ArrowBig);
            if self.if_token_advance(LeftBrace) {
                let body = self.block_stmts();
                self.consume(RightBrace);
                cases.push((expr, FuncBody::Statements(body)))
            } else {
                let body = self.expr();
                self.consume(Comma);
                cases.push((expr, FuncBody::Expression(Box::new(body))))
            }
        }

        let mut def_case = FuncBody::Statements(vec![]);
        if self.if_token_consume(Underscore) {
            self.consume(ArrowBig);
            if self.if_token_consume(LeftBrace) {
                let body = self.block_stmts();
                def_case = FuncBody::Statements(body)
            } else {
                let body = self.expr();
                def_case = FuncBody::Expression(Box::new(body))
            }
        }
        let stmt = Statement::Match {
            cond,
            cases,
            def_case,
        };
        self.consume(RightBrace);
        stmt
    }

    fn mod_stmt(&mut self) -> Statement {
        let src = self.consume(StringLit).lexeme;
        self.consume(Semi);
        Statement::Mod { src }
    }

    fn use_stmt(&mut self) -> Statement {
        let mut names = vec![];
        let mut all = false;
        if self.if_token_advance(Mult) {
            all = true;
            self.consume(From);
        } else {
            while !self.if_token_advance(From) {
                let name = self.consume(Ident);
                if self.if_token_consume(As) {
                    let as_name = self.consume(Ident);
                    names.push((name, Some(as_name)))
                } else {
                    names.push((name, None))
                }
                self.if_token_consume(Comma);
            }
        }
        let src = self.consume(StringLit).lexeme;
        self.consume(Semi);
        Statement::Use { src, names, all }
    }

    fn enum_stmt(&mut self) -> Statement {
        let is_pub = self.if_token_consume(Pub);
        let name = self.consume_uppercase_ident();
        self.consume(LeftBrace);

        let mut enums = vec![];
        while !self.if_token_consume(RightBrace) {
            let enm = self.consume(Ident);
            enums.push(enm);
            if !self.if_token_consume(Comma) && !self.is_token(RightBrace) {
                self.throw_error(E0x201, vec![self.peek().lexeme]);
            }
        }
        Statement::Enum {
            name,
            enums,
            is_pub,
        }
    }

    fn block_stmts(&mut self) -> Vec<Statement> {
        
        match self.block_stmt() {
            Statement::Block { stmts } => {
                self.consume(RightBrace);
                stmts
            }
            _ => self.throw_error(E0x203, vec!["a block statement".to_string()]),
        }
    }

    fn block_stmt(&mut self) -> Statement {
        let mut stmts = vec![];
        while !self.is_token(RightBrace) && !self.is_token(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        Statement::Block { stmts }
    }
}
