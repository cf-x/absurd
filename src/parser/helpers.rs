// helper methods for parser
use super::Parser;
use crate::ast::{
    LiteralKind, LiteralType, Token,
    TokenType::{self, *},
};
use crate::errors::ErrorCode::{self, *};
use colored::Colorize;
use std::process::exit;

impl Parser {
    /// extracts values from token
    pub fn to_value_type(&mut self, token: Token) -> LiteralType {
        match token.token {
            NumberLit => {
                if let Some(LiteralKind::Number { value, .. }) = token.value {
                    LiteralType::Number(value)
                } else {
                    self.throw_error(E0x202, vec![self.peek().lexeme])
                }
            }
            StringLit => {
                if let Some(LiteralKind::String { value }) = token.value {
                    LiteralType::String(value)
                } else {
                    self.throw_error(E0x202, vec![self.peek().lexeme])
                }
            }
            CharLit => {
                if let Some(LiteralKind::Char { value }) = token.value {
                    LiteralType::Char(value)
                } else {
                    self.throw_error(E0x202, vec![self.peek().lexeme])
                }
            }
            TrueLit => LiteralType::Boolean(true),
            FalseLit => LiteralType::Boolean(false),
            Null => LiteralType::Null,
            _ => LiteralType::Null,
        }
    }

    #[inline]
    /// well, checks if token is literal
    pub fn is_literal(&self) -> bool {
        self.are_tokens(&[NumberLit, StringLit, CharLit, TrueLit, FalseLit, Null])
    }

    #[inline]
    /// consumes token if its there
    pub fn if_token_consume(&mut self, token: TokenType) -> bool {
        if self.is_token(token.clone()) {
            self.consume(token);
            true
        } else {
            false
        }
    }

    #[inline]
    /// advance, instead of consuming, if token is here
    pub fn if_token_advance(&mut self, token: TokenType) -> bool {
        if self.is_token(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    #[inline]
    /// checks if identifier is uppercase
    pub fn is_uppercase_ident(&self) -> bool {
        self.peek()
            .lexeme
            .chars()
            .next()
            .map_or(false, |c| c.is_uppercase())
    }

    /// self explanatory
    pub fn consume_uppercase_ident(&mut self) -> Token {
        if self.is_uppercase_ident() {
            self.consume(Ident)
        } else {
            self.throw_error(E0x204, vec!["uppercase Ident".to_string()])
        }
    }

    /// takes multiple tokens and consumes whichever matches first
    pub fn consume_some(&mut self, ts: &[TokenType]) -> Token {
        for t in ts {
            if self.if_token_advance(t.clone()) {
                return self.prev(1);
            }
        }
        self.throw_error(E0x204, vec![self.prev(1).lexeme])
    }

    /// self explanatory
    pub fn consume(&mut self, t: TokenType) -> Token {
        if self.if_token_advance(t.clone()) {
            self.prev(1)
        } else {
            self.throw_error(E0x204, vec![t.to_string()])
        }
    }

    #[inline]
    /// self explanatory
    pub fn advance(&mut self) -> Token {
        if !self.is_token(Eof) {
            self.crnt += 1;
        }
        self.prev(1)
    }

    #[inline]
    /// opposite of advance
    pub fn retreat(&mut self) -> Token {
        if self.crnt > 0 {
            self.crnt -= 1;
        }
        self.prev(1)
    }

    #[inline]
    /// peeks back
    pub fn prev(&self, back: usize) -> Token {
        if self.crnt < back {
            Token {
                token: Eof,
                lexeme: "\0".to_string(),
                line: 0,
                pos: (0, 0),
                value: None,
            }
        } else {
            self.tokens[self.crnt - back].clone()
        }
    }

    #[inline]
    /// checks if multiple tokens are checked
    pub fn are_tokens(&self, tokens: &[TokenType]) -> bool {
        tokens.iter().any(|token| self.is_token(token.clone()))
    }

    #[inline]
    /// checks if token is checked
    pub fn is_token(&self, token: TokenType) -> bool {
        !self.check(Eof) && self.check(token)
    }

    #[inline]
    /// self explanatory
    pub fn check(&self, token: TokenType) -> bool {
        self.peek().token == token
    }

    #[inline]
    /// self explanatory
    pub fn peek(&self) -> Token {
        self.tokens[self.crnt].clone()
    }

    #[inline]
    /// gets id and increases previous
    pub fn id(&mut self) -> usize {
        self.id += 1;
        self.id - 1
    }

    #[inline]
    pub fn start(&self, msg: &str) {
        if self.log {
            println!("      {}{}...", "parsing ".yellow(), msg.blue())
        }
    }

    #[inline]
    pub fn log(&self, msg: &str) {
        if self.log {
            println!("      {}{}", "completed parsing ".green(), msg.blue())
        }
    }

    pub fn throw_error(&mut self, code: ErrorCode, args: Vec<String>) -> ! {
        self.err
            .throw(code, self.peek().line, self.peek().pos, args);
        exit(1);
    }

    pub fn create_null_token(&self, line: usize) -> Token {
        Token {
            token: Null,
            pos: self.peek().pos,
            lexeme: "null".to_string(),
            value: None,
            line,
        }
    }
}
