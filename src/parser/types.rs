use super::Parser;
use crate::utils::errors::ErrorCode::*;
use crate::{
    ast::{LiteralKind, Token, TokenType::*},
    interpreter::types::TypeKind,
};

impl Parser {
    pub fn consume_type(&mut self) -> Token {
        let mut left = self.primary_type();
        if self.if_token_consume(Or) {
            let mut right = self.consume_type();
            let value = Some(LiteralKind::Type(Box::new(TypeKind::Or {
                left: Box::new(left.token_to_typekind()),
                right: Box::new(right.token_to_typekind()),
            })));
            left = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: left.pos,
                value,
                line: left.line,
            };
        } else if self.if_token_consume(Queston) {
            let mut null = Token {
                token: Null,
                lexeme: "null".to_string(),
                pos: left.pos,
                value: Some(LiteralKind::Null),
                line: left.line,
            };
            let value = Some(LiteralKind::Type(Box::new(TypeKind::Or {
                left: Box::new(left.token_to_typekind()),
                right: Box::new(null.token_to_typekind()),
            })));
            left = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: left.pos,
                value,
                line: left.line,
            };
        }
        left
    }

    fn primary_type(&mut self) -> Token {
        match self.peek().token {
            LeftBrace => self.parse_obj_type(),
            Less => self.parse_array_type(),
            Pipe => self.parse_func_type(),
            StringLit | NumberLit | CharLit | Null | TrueLit | ArrayLit | FalseLit => {
                self.parse_literal_type()
            }
            Ident => self.parse_ident_type(),
            AnyIdent | BoolIdent | CharIdent | VoidIdent | ArrayIdent | NumberIdent
            | StringIdent => self.parse_builtin_type(),
            c => Token {
                token: c,
                lexeme: self.peek().lexeme.clone(),
                pos: self.peek().pos,
                value: None,
                line: self.peek().line,
            },
        }
    }

    fn parse_obj_type(&mut self) -> Token {
        let mut fields = vec![];
        self.consume(LeftBrace);
        while !self.if_token_consume(RightBrace) {
            let ident = self.consume(Ident);
            self.consume(Colon);

            let value = self.consume_type();
            fields.push((ident, TypeKind::Var { name: value }));
            if !self.if_token_consume(Comma) {
                self.consume(RightBrace);
                break;
            }
        }
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Obj {
            fields: fields.clone(),
        })));
        let s: String = fields
            .iter()
            .map(|(i, v)| format!("{}: {}, ", i.lexeme.clone(), v.clone()))
            .collect();
        Token {
            token: Type,
            lexeme: format!("{{ {}}}", s),
            value,
            line: self.peek().line,
            pos: self.peek().pos,
        }
    }

    fn parse_builtin_type(&mut self) -> Token {
        let token = self.consume_some(&[
            AnyIdent,
            BoolIdent,
            CharIdent,
            Null,
            VoidIdent,
            ArrayIdent,
            NumberIdent,
            StringIdent,
        ]);
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Var {
            name: token.clone(),
        })));
        Token {
            token: token.token,
            lexeme: token.lexeme,
            value,
            line: token.line,
            pos: token.pos,
        }
    }

    fn parse_ident_type(&mut self) -> Token {
        let token = self.consume(Ident);
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Var {
            name: token.clone(),
        })));
        Token {
            token: Ident,
            lexeme: token.lexeme,
            value,
            line: token.line,
            pos: token.pos,
        }
    }

    fn parse_literal_type(&mut self) -> Token {
        let token = self.peek();
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Value {
            kind: token.value.clone().unwrap_or(LiteralKind::Null),
        })));
        self.advance();
        Token {
            token: token.token,
            lexeme: token.lexeme,
            value,
            line: token.line,
            pos: token.pos,
        }
    }

    fn parse_func_type(&mut self) -> Token {
        self.consume(Pipe);
        let mut params: Vec<TypeKind> = vec![];
        while !self.if_token_consume(Pipe) {
            let param = self.consume_type();
            params.push(TypeKind::Var {
                name: param.clone(),
            });
            if !self.if_token_consume(Comma) {
                break;
            }
        }
        self.advance();
        let return_type = self.consume_type();
        Token {
            token: AnyIdent,
            lexeme: "any".to_string(),
            value: Some(LiteralKind::Type(Box::new(TypeKind::Func {
                params,
                ret: Box::new(TypeKind::Var { name: return_type }),
            }))),
            line: self.peek().line,
            pos: self.peek().pos,
        }
    }

    fn parse_array_type(&mut self) -> Token {
        self.consume(Less);
        if self.if_token_consume(LeftParen) {
            let mut statics: Vec<TypeKind> = vec![];
            while !self.if_token_consume(RightParen) {
                let static_size = self.consume_type();
                statics.push(TypeKind::Var { name: static_size });
                if !self.if_token_consume(Comma) && !self.is_token(RightParen) {
                    self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
                }
            }
            let typ = self.consume_type();
            self.consume(Greater);
            return Token {
                token: ArrayIdent,
                lexeme: typ.lexeme,
                pos: self.peek().pos,
                value: Some(LiteralKind::Type(Box::new(TypeKind::Array {
                    kind: None,
                    statics: Some(statics),
                }))),
                line: self.peek().line,
            };
        }

        let typ = self.consume_type();
        self.consume(Greater);
        Token {
            token: ArrayIdent,
            lexeme: typ.clone().lexeme,
            pos: self.peek().pos,
            value: Some(LiteralKind::Type(Box::new(TypeKind::Array {
                kind: Some(Box::new(TypeKind::Var { name: typ })),
                statics: None,
            }))),
            line: self.peek().line,
        }
    }
}
