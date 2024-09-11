// Absurd type parser
use super::Parser;
use crate::{
    ast::{LiteralKind, Token, TokenType::*},
    interpreter::types::TypeKind,
};

impl Parser {
    pub fn consume_type(&mut self) -> Token {
        let mut lhs = self.primary_type();
        // T || T
        if self.if_token_consume(Or) {
            let mut right = self.consume_type();
            let value = Some(LiteralKind::Type(Box::new(TypeKind::Either {
                lhs: Box::new(lhs.token_to_typekind()),
                rhs: Box::new(right.token_to_typekind()),
            })));
            lhs = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: lhs.pos,
                value,
                line: lhs.line,
            };
        // T?
        } else if self.if_token_consume(Qstn) {
            lhs = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: lhs.pos,
                value: Some(LiteralKind::Type(Box::new(TypeKind::Maybe {
                    lhs: Box::new(lhs.token_to_typekind()),
                }))),
                line: lhs.line,
            };
        // T!
        } else if self.if_token_consume(Bang) {
            lhs = Token {
                token: Type,
                lexeme: "type".to_string(),
                pos: lhs.pos,
                value: Some(LiteralKind::Type(Box::new(TypeKind::Important {
                    lhs: Box::new(lhs.token_to_typekind()),
                }))),
                line: lhs.line,
            };
        }
        lhs
    }

    fn primary_type(&mut self) -> Token {
        match self.peek().token {
            // Record<{i: T, i: T}>
            Record => self.object(),
            // Vec<T>
            VecT => self.vec(),
            // Tuple<(T, T)>
            Tuple => self.tuple(),
            // |i, i| i
            Pipe => self.callback(),
            // literal types
            StrLit | NumLit | CharLit | Null | TrueLit | FalseLit => self.literal(),
            // standard types
            AnyIdent | BoolIdent | CharIdent | VoidIdent | ArrayIdent | NumIdent | StrIdent => {
                self.builtin()
            }
            // for calling aliases
            Ident => self.ident(),
            c => Token {
                token: c,
                lexeme: self.peek().lexeme.clone(),
                pos: self.peek().pos,
                value: None,
                line: self.peek().line,
            },
        }
    }

    fn object(&mut self) -> Token {
        // Record<{i: T, i: T}>
        let mut fields = vec![];
        self.consume(Record);
        self.consume(Ls);
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
        self.consume(Gr);
        let value = Some(LiteralKind::Type(Box::new(TypeKind::Record {
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

    fn builtin(&mut self) -> Token {
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

    fn ident(&mut self) -> Token {
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

    fn literal(&mut self) -> Token {
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

    fn callback(&mut self) -> Token {
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
            token: FuncIdent,
            lexeme: "callback".to_string(),
            value: Some(LiteralKind::Type(Box::new(TypeKind::Callback {
                params,
                ret: Box::new(TypeKind::Var { name: return_type }),
            }))),
            line: self.peek().line,
            pos: self.peek().pos,
        }
    }

    fn vec(&mut self) -> Token {
        // Vec<T>
        self.consume(VecT);
        self.consume(Ls);
        let typ = self.consume_type();
        self.consume(Gr);
        Token {
            token: VecLit,
            lexeme: typ.clone().lexeme,
            pos: self.peek().pos,
            value: Some(LiteralKind::Type(Box::new(TypeKind::Vec {
                kind: Box::new(TypeKind::Var { name: typ }),
            }))),
            line: self.peek().line,
        }
    }

    fn tuple(&mut self) -> Token {
        // Tuple<(T, T)>
        self.consume(Tuple);
        self.consume(Ls);
        self.consume(LParen);
        let mut types = vec![];
        while !self.is_token(RParen) {
            types.push(TypeKind::Var {
                name: self.consume_type(),
            });
            if !self.if_token_consume(Comma) {
                self.consume(RParen);
                break;
            }
        }

        self.consume(Gr);
        Token {
            token: TupleLit,
            lexeme: "tuple".to_string(),
            pos: self.peek().pos,
            value: Some(LiteralKind::Type(Box::new(TypeKind::Tuple { types }))),
            line: self.peek().line,
        }
    }
}
