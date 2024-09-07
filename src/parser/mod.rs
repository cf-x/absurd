// Asburd Parser, transforms tokens into AST
use crate::ast::{FuncBody, LiteralKind, LiteralType, Statement, Token, TokenType::*};
use crate::errors::{Error, ErrorCode::*};
use crate::interpreter::expr::Expression;
use colored::Colorize;
mod call;
pub mod expr;
mod helpers;
pub mod scanner;
mod types;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    err: Error,
    crnt: usize,
    log: bool,
    id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, err: Error, log: bool) -> Self {
        Self {
            tokens,
            err,
            log,
            crnt: 0,
            id: 0,
        }
    }

    /// parser entry function
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];
        if self.log {
            println!("  {}", "parsing statements...".yellow())
        }
        // parse statements until the end of file (source)
        while !self.check(Eof) {
            stmts.push(self.stmt());
        }
        if self.log {
            println!(
                "  {}",
                format!("completed parsing {} statements", stmts.len()).green()
            )
        }
        // returns collection of statements
        stmts
    }

    fn stmt(&mut self) -> Statement {
        // advance to consume keywords
        self.advance();
        match self.prev(1).token {
            Let => self.var(),
            Func => self.func(),
            Return => self.returns(),
            If => self.ifs(),
            While => self.whiles(),
            Loop => self.loops(),
            Break => self.breaks(),
            Match => self.matchs(),
            Sh => self.shs(),
            Mod => self.mods(),
            Use => self.uses(),
            Enum => self.enums(),
            LBrace => self.block_stmt(),
            TypeStmt => self.types(),
            _ => self.exprs(),
        }
    }

    fn var(&mut self) -> Statement {
        self.start("variable statement");
        let mut names = vec![];
        let mut pub_names = vec![];
        let is_mut = self.if_token_consume(Mut);
        let mut is_pub = false;
        let mut is_null = false;
        let mut is_arr_dest = false;

        // checks if variable is immutable and consumes `pub` keyword, if its there
        if !is_mut && self.if_token_consume(Pub) {
            is_pub = true;
            // if `(` comes, public names will be parsed
            // example: `let pub(name2) name1 ...`
            if self.if_token_consume(LParen) {
                loop {
                    // allow empty names, to don't publish certain names
                    // `let pub(_, name3) name1, name2 ...
                    if self.if_token_consume(Underscore) {
                        pub_names.push(Token {
                            token: Null,
                            lexeme: "null".to_string(),
                            value: None,
                            line: self.peek().line,
                            pos: self.peek().pos,
                        })
                    } else {
                        let name = self.consume(Ident);
                        pub_names.push(name);
                    }
                    if !self.if_token_consume(Comma) || self.is_token(RParen) {
                        break;
                    }
                }
                self.consume(RParen);
            }
        }

        // if `[` is peeked, variable is destructuring an array,
        // example: `let [item1, item2]: <number> = [0, 1];`
        if self.if_token_consume(LBracket) {
            is_arr_dest = true;
            while !self.if_token_consume(RBracket) {
                // allow empty values
                // example: `let [item1, _]: <number> = [0, 1];`
                // @todo: `let [item1, ..] ...`, `let [.., item1] ...`
                if self.if_token_consume(Underscore) {
                    names.push(Token {
                        token: Null,
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
            self.consume(RBracket);
        } else {
            // normally parse through names.
            // if name ends with `;`, return null
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

        // if no specific public names have been defined,
        // publish by their local names
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
            self.log("variable statement");
            return null_var;
        }

        let mut value_type = Token {
            token: AnyIdent,
            lexeme: "any".to_string(),
            value: None,
            line: 0,
            pos: (0, 0),
        };
        let mut is_inference = false;
        if self.if_token_consume(Colon) {
            value_type = self.consume_type();
            if value_type.token == Null && self.peek().token != Assign {
                self.log("variable statement");
                self.consume(Semi);
                return null_var;
            }
        } else {
            is_inference = true;
        }

        // consume type after `:`
        if self.if_token_consume(Semi) {
            self.log("variable statement");
            // differes from normal `null_var` with dynamic `value_type`
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
        // check if variable has a callback as a value
        let is_func = self.is_token(Pipe);
        let value = self.expr();
        if is_inference {
            value_type = value.to_literal().to_token();
        }
        self.consume(Semi);

        self.log("variable statement");
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

    fn func(&mut self) -> Statement {
        self.start("function statement");
        let mut params = vec![];
        let mut is_async = false;
        let mut is_pub = false;

        // handles both `func pub async...` and `func async pub`...
        if self.if_token_consume(Pub) {
            is_pub = true;
            if self.if_token_consume(Async) {
                is_async = true;
            }
        } else if self.if_token_consume(Async) {
            is_async = true;
            if self.if_token_consume(Pub) {
                is_pub = true;
            }
        }

        let name = self.consume(Ident);

        // handles parameters, `...(i: T, i: T)...`
        self.consume(LParen);
        while !self.if_token_consume(RParen) {
            if self.is_token(Ident) {
                let param_name = self.consume(Ident);
                self.consume(Colon);
                let param_type = self.consume_type();
                params.push((param_name, param_type))
            } else if self.if_token_consume(Comma) {
            } else if !self.is_token(RParen) {
                self.throw_error(E0x103, vec![self.peek().lexeme]);
            }
        }

        // consume function output type
        self.consume(Arrow);
        let value_type = self.consume_type();

        // parse as a short function
        if self.if_token_consume(Assign) {
            let body = self.expr();
            self.consume(Semi);
            self.log("function statement");
            return Statement::Func {
                name,
                value_type,
                body: FuncBody::Statements(vec![Statement::Return { expr: body }]),
                params,
                is_async,
                is_pub,
            };
        }

        // standard block parsing
        self.consume(LBrace);
        let body = self.block_stmts();
        self.log("function statement");
        Statement::Func {
            name,
            value_type,
            body: FuncBody::Statements(body),
            params,
            is_async,
            is_pub,
        }
    }

    fn returns(&mut self) -> Statement {
        self.start("return statement");
        let expr = if self.is_token(Semi) {
            Expression::Value {
                id: self.id(),
                value: LiteralType::Null,
            }
        } else {
            self.expr()
        };
        self.consume(Semi);
        self.log("return statement");
        Statement::Return { expr }
    }

    fn ifs(&mut self) -> Statement {
        self.start("if statement");
        let cond = self.expr();
        let body = self.block_stmts();
        let mut else_if_branches = vec![];
        // parse elifs
        while self.if_token_consume(Elif) {
            let elif_preds = self.expr();
            let elif_stmt = self.block_stmts();
            else_if_branches.push((elif_preds, elif_stmt))
        }

        // parse else, if avaiable
        let else_branch = if self.if_token_consume(Else) {
            Some(self.block_stmts())
        } else {
            None
        };
        self.log("if statement");
        Statement::If {
            cond,
            body,
            else_if_branches,
            else_branch,
        }
    }

    fn whiles(&mut self) -> Statement {
        self.start("while statement");
        // everything is obvious, I guess.
        let cond = self.expr();
        let body = self.block_stmts();
        self.log("while statement");
        Statement::While { cond, body }
    }

    fn loops(&mut self) -> Statement {
        self.start("loop statement");
        // checks if iterator index is there
        let iter = if self.is_token(NumLit) {
            let num = match self.consume(NumLit).value {
                Some(LiteralKind::Number { value, .. }) => value,
                _ => self.throw_error(E0x104, vec![self.peek().lexeme]),
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
        self.log("loop statement");
        Statement::Loop { iter, body }
    }

    fn breaks(&mut self) -> Statement {
        self.start("break statement");
        self.consume(Semi);
        self.log("break statement");
        Statement::Break {}
    }

    fn matchs(&mut self) -> Statement {
        self.start("match statement");
        let cond = self.expr();
        self.consume(LBrace);
        let mut cases = vec![];

        // match can only "match" literals and Enums
        while self.is_literal() || self.is_uppercase_ident() {
            let expr = self.expr();
            self.consume(ArrowBig);
            // consume block
            if self.if_token_advance(LBrace) {
                let body = self.block_stmts();
                cases.push((expr, FuncBody::Statements(body)))
            } else {
                // consume expression
                let body = self.expr();
                self.consume(Comma);
                cases.push((expr, FuncBody::Expression(Box::new(body))))
            }
        }

        let mut def_case = FuncBody::Statements(vec![]);
        // default branch `_ => {}`
        if self.if_token_consume(Underscore) {
            self.consume(ArrowBig);
            if self.if_token_consume(LBrace) {
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
        self.consume(RBrace);
        self.log("match statement");
        stmt
    }

    // very simple syntax
    fn shs(&mut self) -> Statement {
        self.start("sh statement");
        let cmd = self.consume(StrLit).lexeme;
        self.consume(Semi);
        self.log("sh statement");
        Statement::Sh { cmd }
    }

    fn mods(&mut self) -> Statement {
        self.start("mod statement");
        let src = self.consume(StrLit).lexeme;
        self.consume(Semi);
        self.log("mod statement");
        Statement::Mod { src }
    }

    fn uses(&mut self) -> Statement {
        self.start("use statement");
        let mut names = vec![];
        let mut all = false;
        // `use * from ""`, imports everything
        if self.if_token_advance(Mul) {
            all = true;
            self.consume(From);
        } else {
            // use i from ""
            // use i, i from ""
            // use i as i, ii from ""
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
        let src = self.consume(StrLit).lexeme;
        self.consume(Semi);
        self.log("use statement");
        Statement::Use { src, names, all }
    }

    fn enums(&mut self) -> Statement {
        self.start("enum statement");
        let is_pub = self.if_token_consume(Pub);
        let name = self.consume_uppercase_ident();
        self.consume(LBrace);

        let mut enums = vec![];
        // block with uppercase identifiers
        while !self.if_token_consume(RBrace) {
            let enm = self.consume(Ident);
            enums.push(enm);
            if !self.if_token_consume(Comma) && !self.is_token(RBrace) {
                self.throw_error(E0x103, vec![self.peek().lexeme]);
            }
        }

        self.log("enum statement");
        Statement::Enum {
            name,
            enums,
            is_pub,
        }
    }

    fn types(&mut self) -> Statement {
        self.start("type statement");
        let is_pub = self.if_token_consume(Pub);
        let name = self.consume(Ident);
        self.consume(Assign);
        let value = self.consume_type();
        self.consume(Semi);
        self.log("type statement");
        Statement::Type {
            name,
            value,
            is_pub,
        }
    }

    fn block_stmts(&mut self) -> Vec<Statement> {
        match self.block_stmt() {
            Statement::Block { stmts } => {
                self.consume(RBrace);
                stmts
            }
            _ => self.throw_error(E0x105, vec!["a block statement".to_string()]),
        }
    }

    fn block_stmt(&mut self) -> Statement {
        self.start("block statement");
        let mut stmts = vec![];
        while !self.is_token(RBrace) && !self.is_token(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        self.log("block statement");
        Statement::Block { stmts }
    }
}
