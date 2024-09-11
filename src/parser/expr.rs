// parses expressions
use super::Parser;
use crate::ast::{FuncBody, Statement, Token, TokenType::*};
use crate::errors::ErrorCode::{E0x103, E0x107};
use crate::interpreter::expr::{AssignKind, Expression};

impl Parser {
    pub fn exprs(&mut self) -> Statement {
        // retreat consumed keyword
        self.retreat();
        let expr = self.expr();
        self.consume(Semi);
        Statement::Expression { expr }
    }
    // goes from most to least important expression
    pub fn expr(&mut self) -> Expression {
        let expr = self.binary();
        self.advance();
        match self.prev(1).token {
            // assignments
            Assign => self.assign(&expr, AssignKind::Normal),
            PlusEq => self.assign(&expr, AssignKind::Plus),
            MinEq => self.assign(&expr, AssignKind::Minus),
            MulEq => self.assign(&expr, AssignKind::Mult),
            DivEq => self.assign(&expr, AssignKind::Div),
            _ => {
                self.retreat();
                expr
            }
        }
    }

    pub fn primary(&mut self) -> Expression {
        let token = self.peek().clone();
        match token.token {
            Ident => {
                self.advance();
                Expression::Var {
                    id: self.id(),
                    name: token,
                }
            }
            LBracket => {
                self.advance();
                self.arr_expr()
            }
            LBrace => {
                self.advance();
                self.obj_expr()
            }
            LParen => {
                if self.prev(1).token == Ident {
                    self.advance();
                    self.func_call()
                } else {
                    self.group_expr()
                }
            }
            Pipe => self.func_expr(),
            Await => self.await_expr(),
            If => self.if_expr(),
            _ => {
                if self.is_literal() {
                    self.advance();
                    Expression::Value {
                        id: self.id(),
                        value: self.to_value_type(token),
                    }
                } else {
                    self.throw_error(E0x103, vec![self.peek().lexeme.clone()]);
                }
            }
        }
    }

    fn if_expr(&mut self) -> Expression {
        self.consume(If);
        let cond = self.expr();
        self.consume(Colon);
        let body = self.expr();
        let mut else_branch = None;
        if self.if_token_consume(Qstn) {
            else_branch = Some(Box::new(self.expr()))
        }

        Expression::If {
            id: self.id(),
            cond: Box::new(cond),
            body: Box::new(body),
            else_branch,
        }
    }

    fn unary(&mut self) -> Expression {
        if self.are_tokens(&[Bang, DblBang, Qstn, Decr, Incr, Min]) {
            self.advance();
            let operator = self.prev(1).clone();
            let rhs = self.unary();
            let id = self.id();
            Expression::Unary {
                id,
                left: Box::new(rhs),
                operator,
            }
        } else {
            self.call()
        }
    }

    fn binary(&mut self) -> Expression {
        let mut expr = self.unary();
        while self.are_tokens(&[
            Plus, Min, Mul, Div, Prcnt, DblAnd, Or, Eq, BangEq, Gr, GrOrEq, Ls, LsOrEq, Sqr, And,
        ]) {
            self.advance();
            let operator = self.prev(1).clone();
            let rhs = self.unary();
            expr = Expression::Binary {
                id: self.id(),
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            };
        }
        expr
    }

    fn assign(&mut self, expr: &Expression, kind: AssignKind) -> Expression {
        let value = self.expr();
        if let Expression::Var { name, .. } = expr {
            Expression::Assign {
                id: self.id(),
                name: name.clone(),
                value: Box::new(value),
                kind,
            }
        } else {
            self.throw_error(E0x107, vec![]);
        }
    }

    fn obj_expr(&mut self) -> Expression {
        let mut fields = vec![];
        while !self.if_token_consume(RBrace) {
            let key = self.consume(Ident).clone();
            self.consume(Colon);
            let value = self.expr();
            fields.push((key.lexeme, value));
            if !self.if_token_consume(Comma) && !self.is_token(RBrace) {
                self.throw_error(E0x103, vec![self.peek().lexeme.clone()]);
            }
        }
        Expression::Object {
            id: self.id(),
            fields,
        }
    }

    fn arr_expr(&mut self) -> Expression {
        let mut items = vec![];
        while !self.if_token_consume(RBracket) {
            let e = self.expr();
            items.push(e);
            if !self.if_token_consume(Comma) && !self.is_token(RBracket) {
                self.throw_error(E0x103, vec![self.peek().lexeme.clone()]);
            }
        }
        Expression::Vec {
            id: self.id(),
            items,
        }
    }

    fn group_expr(&mut self) -> Expression {
        self.advance();
        let expr = self.expr();
        self.consume(RParen);
        Expression::Grouping {
            id: self.id(),
            expression: Box::new(expr),
        }
    }

    fn func_expr(&mut self) -> Expression {
        self.advance();
        let mut is_inline = false;
        let mut value_type = self.prev(3).clone();
        let mut params = vec![];
        let is_async = false;
        let mut is_pub = false;
        let add = if params.len() > 1 {
            params.len() * 2 - 1
        } else {
            params.len()
        };

        if self.prev(9 + add).token == Pub {
            is_pub = true;
        }
        let mut name = self.prev(10 + add).clone();
        if self.if_token_consume(Underscore) {
            self.consume(Pipe);
        } else {
            while !self.if_token_consume(Pipe) {
                if self.is_token(Ident) {
                    let param_name = self.consume(Ident);
                    if self.if_token_consume(Colon) {
                        let param_type = self.consume_type();
                        params.push((param_name, param_type));
                        is_inline = true;
                    } else {
                        let param_type = self.prev(8).clone();
                        params.push((param_name, param_type))
                    }
                } else if self.if_token_consume(Comma) {
                } else {
                    self.throw_error(E0x103, vec![self.peek().lexeme.clone()]);
                }
            }
        }
        if is_inline {
            value_type = self.consume_type();
            name = Token {
                token: Ident,
                lexeme: "func".to_string(),
                value: None,
                line: 0,
                pos: (0, 0),
            };
        }
        if self.if_token_consume(Colon) {
            let body = self.expr();
            return Expression::Func {
                id: self.id(),
                name,
                value_type,
                body: FuncBody::Expression(Box::new(body)),
                params,
                is_async,
                is_pub,
            };
        }
        self.consume(LBrace);
        let body = self.block_stmts();
        Expression::Func {
            id: self.id(),
            name,
            value_type,
            body: FuncBody::Statements(body),
            params,
            is_async,
            is_pub,
        }
    }

    fn await_expr(&mut self) -> Expression {
        let expr = self.expr();
        Expression::Await {
            id: self.id(),
            expr: Box::new(expr),
        }
    }
}
