use super::Parser;
use crate::ast::{CallType, FuncBody, Statement, TokenType::*};
use crate::interpreter::expr::{AssignKind, Expression};
use crate::utils::errors::ErrorCode::*;

impl Parser {
    pub fn expr_stmt(&mut self) -> Statement {
        self.retreat();
        let expr = self.expr();
        self.consume(Semi);
        Statement::Expression { expr }
    }

    pub fn expr(&mut self) -> Expression {
        let expr = self.binary();
        self.advance();
        match self.prev(1).token {
            Assign => self.assign(&expr, AssignKind::Normal),
            PlusEq => self.assign(&expr, AssignKind::Plus),
            MinEq => self.assign(&expr, AssignKind::Minus),
            MultEq => self.assign(&expr, AssignKind::Mult),
            DivEq => self.assign(&expr, AssignKind::Div),
            _ => {
                self.retreat();
                expr
            }
        }
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
            self.throw_error(E0x201, vec!["Invalid assignment target".to_string()]);
        }
    }

    fn binary(&mut self) -> Expression {
        let mut expr = self.unary();
        while self.are_tokens(&[
            Plus,
            Minus,
            Mult,
            Divide,
            Percent,
            AndAnd,
            Or,
            Eq,
            NotEq,
            Greater,
            GreaterOrEq,
            Less,
            LessOrEq,
            Square,
            And,
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

    fn unary(&mut self) -> Expression {
        if self.are_tokens(&[Not, NotNot, Queston, Decr, Increment, Minus]) {
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

    fn call(&mut self) -> Expression {
        if self.is_token(LeftBracket) && self.prev(1).token == Ident {
            self.advance();
            let arr = self.array_call();
            self.consume(RightBracket);
            return arr;
        }
        let mut expr = self.method();

        while let Some(token) = {
            self.advance();
            Some(self.prev(1).token)
        } {
            match token {
                Dot => {
                    if let Expression::Method { .. } = expr {
                        expr = self.method_body(expr);
                    } else {
                        expr = self.struct_call();
                    }
                }
                DblColon => {
                    expr = self.enum_call();
                }
                LeftParen => {
                    expr = self.func_call();
                }
                LeftBracket => {
                    expr = self.array_call();
                }
                Ident => {
                    expr = self.call();
                }
                _ => {
                    self.retreat();
                    break;
                }
            }
        }
        expr
    }

    fn array_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];
        self.consume(RightBracket);
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Array,
        }
    }

    fn struct_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Struct,
        }
    }

    fn enum_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];

        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Enum,
        }
    }

    fn func_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let mut args = vec![];
        while !self.is_token(RightParen) {
            let arg = self.expr();
            args.push(arg);
            if self.is_token(RightParen) {
                break;
            }
            if !self.if_token_consume(Comma) && !self.is_token(RightParen) {
                self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
            }
        }
        self.consume(RightParen);
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Func,
        }
    }

    fn method(&mut self) -> Expression {
        let mut expr = self.primary();
        if self.if_token_consume(Dot) {
            expr = self.method_body(expr);
        }
        expr
    }

    fn method_body(&mut self, expr: Expression) -> Expression {
        let name = self.consume(Ident).clone();
        self.consume(LeftParen);
        let mut args = vec![];
        while !self.if_token_consume(RightParen) {
            let e = self.expr();
            args.push(e);
            self.if_token_consume(Comma);
        }
        Expression::Method {
            id: self.id(),
            name,
            left: Box::new(expr),
            args,
        }
    }

    fn primary(&mut self) -> Expression {
        let token = self.peek().clone();
        match token.token {
            Ident => {
                self.advance();
                Expression::Var {
                    id: self.id(),
                    name: token,
                }
            }
            LeftBracket => {
                self.advance();
                self.arr_expr()
            }
            LeftParen => {
                if self.prev(1).token == Ident {
                    self.advance();
                    self.func_call()
                } else {
                    self.group_expr()
                }
            }
            Pipe => self.func_expr(),
            Await => self.await_expr(),
            _ => {
                if self.is_literal() {
                    self.advance();
                    Expression::Value {
                        id: self.id(),
                        value: self.to_value_type(token),
                    }
                } else {
                    self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
                }
            }
        }
    }

    fn arr_expr(&mut self) -> Expression {
        let mut items = vec![];
        while !self.if_token_consume(RightBracket) {
            let e = self.expr();
            items.push(e);
            if !self.if_token_consume(Comma) && !self.is_token(RightBracket) {
                self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
            }
        }
        Expression::Array {
            id: self.id(),
            items,
        }
    }

    fn group_expr(&mut self) -> Expression {
        self.advance();
        let expr = self.expr();
        self.consume(RightParen);
        Expression::Grouping {
            id: self.id(),
            expression: Box::new(expr),
        }
    }

    fn func_expr(&mut self) -> Expression {
        self.advance();
        let value_type = self.prev(3).clone();
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
        let name = self.prev(10 + add).clone();
        if self.if_token_consume(Underscore) {
            self.consume(Pipe);
        } else {
            while !self.if_token_consume(Pipe) {
                if self.is_token(Ident) {
                    let param_name = self.consume(Ident).clone();
                    let param_type = self.prev(8).clone();
                    params.push((param_name, param_type))
                } else if self.if_token_consume(Comma) {
                } else if !self.is_token(Pipe) {
                    self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
                }
            }
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
        self.consume(LeftBrace);
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
