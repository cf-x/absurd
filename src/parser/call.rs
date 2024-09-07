use super::Parser;
use crate::ast::CallType;
use crate::ast::LiteralType;
use crate::ast::TokenType::*;
use crate::errors::ErrorCode::E0x103;
use crate::interpreter::expr::Expression;

impl Parser {
    pub fn array_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];
        self.consume(RBracket);
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

    pub fn obj_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.consume(Ident);
        let args = vec![Expression::Value {
            id: self.id(),
            value: LiteralType::String(e.lexeme),
        }];
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

    pub fn enum_call(&mut self) -> Expression {
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

    pub fn func_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let mut args = vec![];
        while !self.is_token(RParen) {
            let arg = self.expr();
            args.push(arg);
            if self.is_token(RParen) {
                break;
            }
            if !self.if_token_consume(Comma) && !self.is_token(RParen) {
                self.throw_error(E0x103, vec![self.peek().lexeme.clone()]);
            }
        }
        self.consume(RParen);
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

}
