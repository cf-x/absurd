// Asburd Parser, transforms tokens into AST
use crate::ast::{Destruct, LiteralType, RecordField, Statement, Token, TokenType::*};
use crate::errors::{raw, Error, ErrorCode::*};
use crate::interpreter::expr::Expression;
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
    id: usize,
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

    /// parser entry function
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];

        // parse statements until the end of file (source)
        while !self.check(Eof) {
            stmts.push(self.stmt());
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
            For => self.fors(),
            While => self.whiles(),
            Break => self.breaks(),
            Match => self.matchs(),
            Sh => self.shs(),
            Mod => self.mods(),
            Use => self.uses(),
            LBrace => self.block_stmt(),
            TypeStmt => self.types(),
            RecordStmt => self.record(),
            Enum => self.enums(),
            Label => self.label(),
            _ => self.exprs(),
        }
    }

    fn label(&mut self) -> Statement {
        self.if_token_consume(Ident);
        self.consume(Colon);
        self.stmt()
    }

    /// parses variable publicity and returns variable publicit
    fn var_is_pub(&mut self, is_mut: bool) -> Vec<Token> {
        let mut pub_names = vec![];
        if !is_mut && self.if_token_consume(Pub) {
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
        pub_names
    }

    /// parses vector destruction
    fn var_vec_dest(&mut self) -> Vec<Token> {
        let mut names = vec![];
        while !self.if_token_consume(RBracket) {
            // allow empty values: [a, _, c]
            if self.if_token_consume(Underscore) {
                names.push(Token::null());
            // [a, ..]
            } else if self.if_token_consume(DblDot) {
                names.push(Token::null());
                if !self.is_token(Comma) {
                    break;
                }
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
        names
    }

    /// parses record destruction
    fn var_record_dest(&mut self) -> Vec<Token> {
        let mut names = vec![];
        while !self.if_token_consume(RBrace) {
            // {name, ..}
            if self.if_token_consume(DblDot) {
                names.push(Token::null());
                if !self.is_token(Comma) {
                    break;
                }
            } else {
                let name = self.consume(Ident);
                names.push(name);
            }
            if !self.is_token(Comma) || self.is_token(Colon) {
                break;
            }
            self.advance();
        }
        self.consume(RBrace);
        names
    }

    /// parses tuple destruction
    fn var_tuple_dest(&mut self) -> Vec<Token> {
        let mut names = vec![];
        while !self.if_token_consume(LParen) {
            // allow empty values: (a, _)
            if self.if_token_consume(Underscore) {
                names.push(Token::null());
            } else {
                let name = self.consume(Ident);
                names.push(name);
            }
            if !self.is_token(Comma) || self.is_token(Colon) {
                break;
            }
            self.advance();
        }
        self.consume(RParen);
        names
    }

    fn var(&mut self) -> Statement {
        let mut names = vec![];
        let is_mut = self.if_token_consume(Mut);
        let mut is_null = false;
        let mut destruct = None;

        // checks if variable is immutable and consumes `pub` keyword, if its there
        let mut pub_names = self.var_is_pub(is_mut);
        let mut is_pub = !pub_names.is_empty();
        if self.if_token_consume(LBracket) {
            names = self.var_vec_dest();
            destruct = Some(Destruct::Vector)
        } else if self.if_token_consume(LBrace) {
            names = self.var_record_dest();
            destruct = Some(Destruct::Record)
        } else if self.if_token_consume(LParen) {
            names = self.var_tuple_dest();
            destruct = Some(Destruct::Tuple)
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
            is_pub = false;
        }

        let null_var = Statement::Var {
            names: names.clone(),
            value_type: Token::null(),
            value: Some(Expression::Value {
                id: self.id(),
                value: LiteralType::Null,
            }),
            is_mut,
            is_pub,
            pub_names: pub_names.clone(),
            is_func: false,
            destruct: destruct.clone(),
        };

        if is_null {
            self.advance();
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
                self.consume(Semi);
                return null_var;
            }
        } else {
            is_inference = true;
        }

        // consume type after `:`
        if self.if_token_consume(Semi) {
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
                destruct,
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
        Statement::Var {
            names,
            value_type,
            value: Some(value),
            is_mut,
            is_pub,
            pub_names,
            is_func,
            destruct,
        }
    }

    fn func(&mut self) -> Statement {
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
            let expr = self.expr();
            self.consume(Semi);
            return Statement::Func {
                name,
                value_type,
                body: Box::new(Statement::Expression { expr }),
                params,
                is_async,
                is_pub,
            };
        }

        // standard block parsing
        self.consume(LBrace);
        let body = self.block_stmt();
        Statement::Func {
            name,
            value_type,
            body: Box::new(body),
            params,
            is_async,
            is_pub,
        }
    }

    fn returns(&mut self) -> Statement {
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

    fn ifs(&mut self) -> Statement {
        let cond = self.expr();

        let body = if self.if_token_consume(LBrace) {
            self.block_stmt()
        } else {
            self.consume(Colon);
            self.stmt()
        };
        let mut else_if_branches = vec![];
        // parse elifs
        while self.if_token_consume(Elif) {
            let elif_preds = self.expr();
            let elif_stmt = self.block_stmts();
            else_if_branches.push((elif_preds, elif_stmt))
        }

        // parse else, if avaiable
        let else_branch = if self.if_token_consume(Else) {
            Some(Box::new(self.stmt()))
        } else {
            None
        };
        Statement::If {
            cond,
            body: Box::new(body),
            else_branch,
        }
    }

    fn fors(&mut self) -> Statement {
        let iterator = self.consume(Ident);
        let index = if self.if_token_consume(Comma) {
            Some(self.consume(Ident))
        } else {
            None
        };
        self.consume(In);
        let expr = self.expr();
        let body = self.block_stmt();
        Statement::For {
            iterator,
            index,
            expr,
            body: Box::new(body),
        }
    }

    fn whiles(&mut self) -> Statement {
        // everything is obvious, I guess.
        let cond = self.expr();
        let body = self.block_stmt();
        Statement::While {
            cond,
            body: Box::new(body),
        }
    }

    fn breaks(&mut self) -> Statement {
        self.consume(Semi);
        Statement::Break {}
    }

    fn enums(&mut self) -> Statement {
        let name = self.consume(Ident);
        if !self.is_uppercase(name.clone()) {
            raw("enum name must start with uppercase alphabet");
        }

        let is_pub = if self.if_token_consume(Pub) {
            true
        } else {
            false
        };
        self.consume(LBrace);
        let mut items = vec![];
        while !self.if_token_consume(RBrace) {
            let name = self.consume(Ident);
            if !self.is_uppercase(name.clone()) {
                raw("enum name must start with uppercase alphabet");
            }
            if self.if_token_consume(LParen) {
                let typ = self.consume_type();
                self.consume(RParen);
                items.push((name.clone(), Some(typ)))
            }
            items.push((name, None));
            if !self.if_token_consume(Comma) && !self.is_token(RBrace) {
                break;
            }
        }

        Statement::Enum {
            name,
            is_pub,
            items,
        }
    }

    fn matchs(&mut self) -> Statement {
        let cond = self.expr();
        self.consume(LBrace);
        let mut cases = vec![];

        // match can only "match" literals and Enums
        while self.is_literal() || self.is_uppercase_ident() {
            let expr = self.expr();
            self.consume(ArrowBig);
            // consume block
            if self.if_token_advance(LBrace) {
                let body = self.block_stmt();
                cases.push((expr, body))
            } else {
                // consume expression
                let expr = self.expr();
                self.consume(Comma);
                cases.push((expr.clone(), Statement::Expression { expr }))
            }
        }

        let mut def_case = Statement::Break {};
        // default branch `_ => {}`
        if self.if_token_consume(Underscore) {
            self.consume(ArrowBig);
            if self.if_token_consume(LBrace) {
                def_case = self.block_stmt();
            } else {
                let expr = self.expr();
                def_case = Statement::Expression { expr };
            }
        }
        let stmt = Statement::Match {
            cond,
            cases,
            def_case: Box::new(def_case),
        };
        self.consume(RBrace);
        stmt
    }

    // very simple syntax
    fn shs(&mut self) -> Statement {
        let cmd = self.consume(StrLit).lexeme;
        self.consume(Semi);
        Statement::Sh { cmd }
    }

    fn mods(&mut self) -> Statement {
        let src = self.consume(StrLit).lexeme;
        let name = if self.if_token_consume(As) {
            Some(self.consume(Ident).lexeme)
        } else {
            None
        };
        self.consume(Semi);
        Statement::Mod { src, name }
    }

    fn uses(&mut self) -> Statement {
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
        let src = if self.is_token(Ident) {
            let mut lex = self.consume(Ident).lexeme;
            if lex == "std" {
                while self.if_token_consume(DblColon) {
                    let n = self.consume(Ident);
                    lex.push_str("::");
                    lex.push_str(n.lexeme.as_str());
                }
            }
            format!("\"{}\"", lex)
        } else {
            self.consume(StrLit).lexeme
        };
        self.consume(Semi);
        Statement::Use { src, names, all }
    }

    fn types(&mut self) -> Statement {
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

    fn record(&mut self) -> Statement {
        let name = self.consume(Ident);

        let mut extends = vec![];
        if self.if_token_consume(Extends) {
            extends.push(self.consume(Ident));

            if self.if_token_consume(Comma) {
                while !self.if_token_consume(Comma) {
                    extends.push(self.consume(Ident));
                    if !self.if_token_consume(Comma) {
                        break;
                    }
                }
            }
        }

        let mut fields = vec![];
        self.consume(LBrace);

        while !self.if_token_consume(RBrace) {
            let name = self.consume(Ident);
            let mut is_strict = false;
            let mut is_optional = false;
            if self.if_token_consume(Qstn) {
                is_optional = true;
            } else if self.if_token_consume(Bang) {
                is_strict = true;
            }

            self.consume(Colon);
            let value = self.consume_type();
            let mut default_value = None;

            if self.if_token_consume(Eq) {
                default_value = Some(self.expr())
            }

            fields.push(RecordField {
                name,
                is_strict,
                is_optional,
                value,
                default_value,
            });

            if !self.if_token_consume(Comma) {
                self.consume(RBrace);
                break;
            }
        }

        Statement::Record {
            name,
            extends,
            is_strict: false,
            fields,
        }
    }

    fn block_stmts(&mut self) -> Vec<Statement> {
        match self.block_stmt() {
            Statement::Block { stmts } => stmts,
            _ => self.throw_error(E0x105, vec!["a block statement".to_string()]),
        }
    }

    fn block_stmt(&mut self) -> Statement {
        let mut stmts = vec![];
        while !self.if_token_consume(RBrace) && !self.is_token(Eof) {
            let stmt = self.stmt();
            stmts.push(stmt);
        }
        Statement::Block { stmts }
    }
}
