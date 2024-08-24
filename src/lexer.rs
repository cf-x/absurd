use crate::{ast::{
    Base, LiteralKind, Token,
    TokenType::{self, *},
}, manifest::Project};
use crate::errors::{Error, ErrorCode::*};
use std::collections::HashMap;
use unicode_xid::UnicodeXID;

#[derive(Debug, Clone)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    kwds: HashMap<&'static str, TokenType>,
    line: usize,
    pos: usize,
    start: usize,
    crnt: usize,
    err: Error,
}

impl Lexer {
    pub fn new(source: String, err: Error, _project: Project) -> Self {
        Self {
            source: source.chars().collect(),
            err,
            tokens: vec![],
            kwds: kwds(),
            line: 1,
            pos: 1,
            start: 0,
            crnt: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start = self.crnt;
            self.advance_token()
        }
        self.tokens.push(Token {
            token: Eof,
            lexeme: "\0".to_string(),
            value: None,
            line: self.line,
            pos: (0, 0),
        });
        self.tokens.clone()
    }

    fn is_eof(&self) -> bool {
        self.crnt >= self.source.len()
    }

    fn advance_token(&mut self) {
        let c = self.advance();
        match c {
            '_' => self.push_token(Underscore, None),
            '~' => self.push_token(Tilde, None),
            '%' => self.push_token(Percent, None),
            '(' => self.push_token(LeftParen, None),
            ')' => self.push_token(RightParen, None),
            '{' => self.push_token(LeftBrace, None),
            '}' => self.push_token(RightBrace, None),
            '[' => self.push_token(LeftBracket, None),
            ']' => self.push_token(RightBracket, None),
            ';' => self.push_token(Semi, None),
            ',' => self.push_token(Comma, None),
            '?' => self.push_token(Queston, None),
            ':' => self.handle_colon(),
            '!' => self.handle_not(),
            '&' => self.handle_and(),
            '+' => self.handle_plus(),
            '-' => self.handle_minus(),
            '*' => self.handle_mult(),
            '=' => self.handle_eq(),
            '|' => self.handle_pipe(),
            '.' => self.handle_dot(),
            '<' => self.handle_ls(),
            '>' => self.handle_gr(),
            '\\' => self.handle_esc(),
            '/' => self.handle_div(),
            '\r' => {}
            '\t' => {
                self.pos += 4;
            }
            ' ' => {
                self.pos += 1;
            }
            '\n' => {
                self.pos = 1;
                self.line += 1;
            }
            '\'' => self.char(),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(c),
            c if UnicodeXID::is_xid_start(c) || c == '_' => self.ident(),
            _ => self.push_token(Ident, None),
        };
    }

    fn handle_colon(&mut self) {
        let tt = match self.peek() {
            ':' => {
                self.crnt += 1;
                DblColon
            }
            _ => Colon,
        };
        self.push_token(tt, None);
    }

    fn handle_not(&mut self) {
        let tt = match self.peek() {
            '!' => {
                self.crnt += 1;
                NotNot
            }
            '=' => {
                self.crnt += 1;
                NotEq
            }
            _ => Not,
        };
        self.push_token(tt, None)
    }

    fn handle_and(&mut self) {
        let tt = match self.peek() {
            '&' => {
                self.crnt += 1;
                AndAnd
            }
            _ => And,
        };
        self.push_token(tt, None)
    }

    fn handle_plus(&mut self) {
        let tt = match self.peek() {
            '+' => {
                self.crnt += 1;
                Increment
            }
            '=' => {
                self.crnt += 1;
                PlusEq
            }
            _ => Plus,
        };
        self.push_token(tt, None)
    }

    fn handle_minus(&mut self) {
        let tt = match self.peek() {
            '>' => {
                self.crnt += 1;
                Arrow
            }
            '-' => {
                self.crnt += 1;
                Decr
            }
            '=' => {
                self.crnt += 1;
                MinEq
            }
            _ => Minus,
        };

        self.push_token(tt, None)
    }

    fn handle_mult(&mut self) {
        let tt = match self.peek() {
            '*' => {
                self.crnt += 1;
                Square
            }
            '=' => {
                self.crnt += 1;
                MultEq
            }
            _ => Mult,
        };
        self.push_token(tt, None)
    }

    fn handle_eq(&mut self) {
        let tt = match self.peek() {
            '=' => {
                self.crnt += 1;
                Eq
            }
            '>' => {
                self.crnt += 1;
                ArrowBig
            }
            _ => Assign,
        };
        self.push_token(tt, None)
    }

    fn handle_pipe(&mut self) {
        let tt = match self.peek() {
            '|' => {
                self.crnt += 1;
                Or
            }
            _ => Pipe,
        };
        self.push_token(tt, None)
    }

    fn handle_dot(&mut self) {
        let tt = match self.peek() {
            '.' => {
                self.crnt += 1;
                DotDot
            }
            _ => Dot,
        };
        self.push_token(tt, None)
    }

    fn handle_ls(&mut self) {
        let tt = match self.peek() {
            '=' => {
                self.crnt += 1;
                LessOrEq
            }
            _ => Less,
        };
        self.push_token(tt, None)
    }

    fn handle_gr(&mut self) {
        let tt = match self.peek() {
            '=' => {
                self.crnt += 1;
                GreaterOrEq
            }
            _ => Greater,
        };
        self.push_token(tt, None)
    }

    fn handle_esc(&mut self) {
        let tt = match self.peek() {
            '{' => {
                self.crnt += 1;
                StartParse
            }
            '}' => {
                self.crnt += 1;
                EndParse
            }
            _ => Escape,
        };
        self.push_token(tt, None)
    }

    fn handle_div(&mut self) {
        if self.peek() == '/' {
            self.comment();
        } else if self.peek() == '*' {
            self.block_comment();
        } else {
            let tt = match self.peek() {
                '=' => {
                    self.crnt += 1;
                    DivEq
                }
                _ => Divide,
            };
            self.push_token(tt, None)
        }
    }

    fn comment(&mut self) {
        loop {
            if self.peek() == '\n' || self.is_eof() {
                self.pos = 1;
                break;
            }
            self.advance();
            self.pos += 1;
        }
    }

    fn block_comment(&mut self) {
        loop {
            if self.is_eof() {
                break;
            }

            if self.peek() == '*' {
                self.advance();
                self.pos += 1;
                if self.peek() == '/' {
                    self.advance();
                    self.pos += 1;
                    break;
                }
            } else {
                self.pos += 1;
                self.advance();
            }
        }
    }

    fn char(&mut self) {
        let value = if self.peek() != '\'' && !self.is_eof() {
            let c = self.peek();
            self.advance();
            c
        } else {
            self.err
                .throw(E0x102, self.line, (self.pos - 1, self.pos), vec![]);
            return;
        };
        if self.peek() != '\'' {
            self.err
                .throw(E0x102, self.line, (self.pos - 1, self.pos), vec![]);
            return;
        }
        self.advance();
        self.push_token(CharLit, Some(LiteralKind::Char { value }));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
                self.pos = 1;
            }
            self.advance();
        }
        if self.is_eof() {
            self.err
                .throw(E0x103, self.line, (self.pos - 1, self.pos), vec![]);
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.crnt - 1].iter().collect();
        self.push_token(
            StringLit,
            Some(LiteralKind::String {
                value: value.clone(),
            }),
        )
    }

    fn ident(&mut self) {
        while !self.is_eof() {
            let c = self.peek();
            if UnicodeXID::is_xid_continue(c) || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let sub: String = self.source[self.start..self.crnt].iter().collect();
        let token = self.kwds.get(&sub as &str).clone().unwrap_or(&Ident);
        self.push_token(token.clone(), None);
    }

    fn number(&mut self, c: char) {
        if c == '0' {
            match self.peek() {
                'b' => self.parse_binary(),
                'o' => self.parse_octal(),
                'x' => self.parse_hexadecimal(),
                '0'..='9' | '_' | '.' => self.parse_decimal(),
                _ => self.push_token(
                    NumberLit,
                    Some(LiteralKind::Number {
                        base: Base::Decimal,
                        value: 0.0,
                    }),
                ),
            }
        } else {
            self.parse_decimal()
        }
    }

    fn parse_decimal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let sub: String = self.source[self.start..self.crnt].iter().collect();
        let val = sub.parse::<f32>();
        match val {
            Ok(value) => self.push_token(
                NumberLit,
                Some(LiteralKind::Number {
                    base: Base::Decimal,
                    value,
                }),
            ),
            Err(_) => self.err.throw(
                E0x104,
                self.line,
                (self.pos - 1, self.pos),
                vec!["decimal".to_string(), sub],
            ),
        }
    }
    fn parse_binary(&mut self) {
        self.advance();
        self.advance();

        while self.peek().is_digit(2) {
            self.advance();
        }

        let sub: String = self.source[self.start..self.crnt].iter().collect();
        let val = i32::from_str_radix(&sub[2..], 2);
        match val {
            Ok(value) => self.push_token(
                NumberLit,
                Some(LiteralKind::Number {
                    base: Base::Binary,
                    value: value as f32,
                }),
            ),
            Err(_) => self.err.throw(
                E0x104,
                self.line,
                (self.pos - 1, self.pos),
                vec!["binary".to_string(), sub],
            ),
        }
    }
    fn parse_octal(&mut self) {
        self.advance();
        self.advance();

        while self.peek().is_digit(8) {
            self.advance();
        }

        let sub: String = self.source[self.start..self.crnt].iter().collect();
        let val = i32::from_str_radix(&sub[2..], 8);
        match val {
            Ok(value) => self.push_token(
                NumberLit,
                Some(LiteralKind::Number {
                    base: Base::Octal,
                    value: value as f32,
                }),
            ),
            Err(_) => self.err.throw(
                E0x104,
                self.line,
                (self.pos - 1, self.pos),
                vec!["octal".to_string(), sub],
            ),
        }
    }
    fn parse_hexadecimal(&mut self) {
        self.advance();
        self.advance();

        while self.peek().is_digit(16) {
            self.advance();
        }
        let sub: String = self.source[self.start..self.crnt].iter().collect();
        let val = i32::from_str_radix(&sub[2..], 16);
        match val {
            Ok(value) => self.push_token(
                NumberLit,
                Some(LiteralKind::Number {
                    base: Base::Hexadecimal,
                    value: value as f32,
                }),
            ),
            Err(_) => self.err.throw(
                E0x104,
                self.line,
                (self.pos - 1, self.pos),
                vec!["hexadecimal".to_string(), sub],
            ),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.get(self.crnt).copied().unwrap_or('\0');
        self.crnt += 1;
        c
    }

    fn push_token(&mut self, token: TokenType, value: Option<LiteralKind>) {
        let lexeme: String = self.source[self.start..self.crnt].iter().collect();
        let pos = (self.pos, self.pos + lexeme.chars().count());
        self.pos += lexeme.chars().count();
        self.tokens.push(Token {
            token,
            lexeme,
            line: self.line,
            value,
            pos,
        })
    }

    fn peek(&self) -> char {
        self.source.get(self.crnt).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.get(self.crnt + 1).copied().unwrap_or('\0')
    }
}

pub fn kwds() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("let", Let),
        ("if", If),
        ("else", Else),
        ("elif", ElseIf),
        ("return", Return),
        ("while", While),
        ("loop", Loop),
        ("break", Break),
        ("match", Match),
        ("mod", Mod),
        ("use", Use),
        ("as", As),
        ("from", From),
        ("struct", Struct),
        ("impl", Impl),
        ("enum", Enum),
        ("async", Async),
        ("await", Await),
        ("pub", Pub),
        ("mut", Mut),
        ("func", Func),
        ("self", Slf),
        ("true", TrueLit),
        ("false", FalseLit),
        ("null", NullLit),
        ("number", NumberIdent),
        ("string", StringIdent),
        ("char", CharIdent),
        ("bool", BoolIdent),
        ("null", NullIdent),
        ("void", VoidIdent),
        ("array", ArrayIdent),
        ("any", AnyIdent),
    ])
}
