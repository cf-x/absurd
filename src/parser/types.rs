// Absurd type parser
use super::Parser;
use crate::{
    ast::{LiteralKind, Token, TokenType::*},
    interpreter::types::TypeKind,
};

impl Parser {
    pub fn consume_type(&mut self) -> Token {
        let mut left = self.primary_type();
        // T || T
        if self.if_token_consume(Or) {
            let mut right = self.consume_type();
            let value = Some(LiteralKind::Type(Box::new(TypeKind::Either {
                lhs: Box::new(left.token_to_typekind()),
                rhs: Box::new(right.token_to_typekind()),
            })));
            left = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: left.pos,
                value,
                line: left.line,
            };
        // T?
        } else if self.if_token_consume(Qstn) {
            let mut null = Token {
                token: Null,
                lexeme: "null".to_string(),
                pos: left.pos,
                value: Some(LiteralKind::Null),
                line: left.line,
            };
            let value = Some(LiteralKind::Type(Box::new(TypeKind::Either {
                lhs: Box::new(left.token_to_typekind()),
                rhs: Box::new(null.token_to_typekind()),
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
            // {i: T, i: T}
            LBrace => self.parse_obj_type(),
            // <i>
            Ls => self.parse_array_type(),
            // |i, i| i
            Pipe => self.parse_func_type(),
            // literal types
            StrLit | NumLit | CharLit | Null | TrueLit | ArrLit | FalseLit => {
                self.parse_literal_type()
            }
            // for calling aliases
            Ident => self.parse_ident_type(),
            // standard types
            AnyIdent | BoolIdent | CharIdent | VoidIdent | ArrayIdent | NumIdent | StrIdent => {
                self.parse_builtin_type()
            }
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
        self.consume(LBrace);
        while !self.if_token_consume(RBrace) {
            let ident = self.consume(Ident);
            self.consume(Colon);

            let value = self.consume_type();
            fields.push((ident, TypeKind::Var { name: value }));
            if !self.if_token_consume(Comma) {
                self.consume(RBrace);
                break;
            }
        }
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Object {
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
            AnyIdent, BoolIdent, CharIdent, Null, VoidIdent, ArrayIdent, NumIdent, StrIdent,
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
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Literal {
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
            value: Some(LiteralKind::Type(Box::new(TypeKind::Callback {
                params,
                ret: Box::new(TypeKind::Var { name: return_type }),
            }))),
            line: self.peek().line,
            pos: self.peek().pos,
        }
    }

    fn parse_array_type(&mut self) -> Token {
        self.consume(Ls);
        let typ = self.consume_type();
        self.consume(Gr);
        Token {
            token: ArrayIdent,
            lexeme: typ.clone().lexeme,
            pos: self.peek().pos,
            value: Some(LiteralKind::Type(Box::new(TypeKind::Vec {
                kind: Box::new(TypeKind::Var { name: typ }),
            }))),
            line: self.peek().line,
        }
    }
}
