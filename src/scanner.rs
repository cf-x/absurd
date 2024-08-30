use crate::ast::{
    Base, LiteralKind, Token,
    TokenType::{self, *},
};
use crate::utils::errors::{Error, ErrorCode::*};
use colored::Colorize;
use std::collections::HashMap;
use unicode_xid::UnicodeXID;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    kwds: HashMap<&'static str, TokenType>,
    line: usize,
    pos: usize,
    start: usize,
    crnt: usize,
    err: Error,
    log: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, err: Error, log: bool) -> Self {
        Self {
            source,
            err,
            tokens: vec![],
            kwds: kwds(),
            line: 1,
            pos: 1,
            start: 0,
            crnt: 0,
            log,
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        if self.log {
            println!("  {}", "collecting tokens...".yellow())
        }
        while !self.is_eof() {
            self.start = self.crnt;
            self.advance_token();
        }
        self.tokens.push(Token {
            token: Eof,
            lexeme: "\0".to_string(),
            value: None,
            line: self.line,
            pos: (0, 0),
        });
        if self.log {
            println!(
                "  {}",
                format!("completed collecting {} tokens", self.tokens.len()).green()
            )
        }
        &self.tokens
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
            ':' => self.handle_double_char_token(':', Colon, DblColon),
            '!' => self.handle_multiple_char_token(Not, &[('=', NotEq), ('!', NotNot)]),
            '&' => self.handle_double_char_token('&', And, AndAnd),
            '+' => self.handle_multiple_char_token(Plus, &[('+', Increment), ('=', PlusEq)]),
            '-' => {
                self.handle_multiple_char_token(Minus, &[('>', Arrow), ('-', Decr), ('=', MinEq)])
            }
            '*' => self.handle_multiple_char_token(Mult, &[('=', MultEq), ('*', Square)]),
            '=' => self.handle_multiple_char_token(Assign, &[('=', Eq), ('>', ArrowBig)]),
            '|' => self.handle_double_char_token('|', Pipe, Or),
            '.' => self.handle_double_char_token('.', Dot, DotDot),
            '<' => self.handle_double_char_token('=', Less, LessOrEq),
            '>' => self.handle_double_char_token('=', Greater, GreaterOrEq),
            '\\' => self.handle_multiple_char_token(Escape, &[('{', StartParse), ('}', EndParse)]),
            '/' => self.handle_division_or_comment(),
            '#' => self.line_comment(),
            '\r' => {}
            '\t' => self.pos += 4,
            ' ' => self.pos += 1,
            '\n' => {
                self.pos = 1;
                self.line += 1;
            }
            '\'' => self.char_literal(),
            '"' => self.string_literal(),
            c if c.is_ascii_digit() => self.number_literal(c),
            c if UnicodeXID::is_xid_start(c) || c == '_' => self.identifier(),
            _ => self.push_token(Ident, None),
        };
    }

    fn handle_multiple_char_token(&mut self, single: TokenType, variants: &[(char, TokenType)]) {
        let token_type =
            if let Some(&(_, ref token)) = variants.iter().find(|&&(ch, _)| ch == self.peek()) {
                self.advance();
                token.clone()
            } else {
                single
            };
        self.push_token(token_type, None);
    }

    fn handle_double_char_token(&mut self, next_char: char, single: TokenType, double: TokenType) {
        let token_type = if self.peek() == next_char {
            self.advance();
            double
        } else {
            single
        };
        self.push_token(token_type, None);
    }

    fn handle_division_or_comment(&mut self) {
        match self.peek() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            '=' => {
                self.advance();
                self.push_token(DivEq, None);
            }
            _ => self.push_token(Divide, None),
        }
    }

    fn line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_eof() {
            self.advance();
        }
        self.pos = 1;
    }

    fn block_comment(&mut self) {
        while !self.is_eof() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.pos = 1;
            }
            self.advance();
        }
    }

    fn char_literal(&mut self) {
        let value = if self.peek() != '\'' && !self.is_eof() {
            self.advance()
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

    fn string_literal(&mut self) {
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
            return;
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.crnt - 1].to_string();
        self.push_token(StringLit, Some(LiteralKind::String { value }));
    }

    fn identifier(&mut self) {
        while UnicodeXID::is_xid_continue(self.peek()) || self.peek() == '_' {
            self.advance();
        }
        let sub = &self.source[self.start..self.crnt];
        let token = self.kwds.get(sub).cloned().unwrap_or(Ident);
        self.push_token(token, None);
    }

    fn number_literal(&mut self, c: char) {
        if c == '0' {
            match self.peek() {
                'b' => self.parse_number_literal(2, Base::Binary),
                'o' => self.parse_number_literal(8, Base::Octal),
                'x' => self.parse_number_literal(16, Base::Hexadecimal),
                '0'..='9' | '_' | '.' => self.parse_number_literal(10, Base::Decimal),
                _ => self.push_token(
                    NumberLit,
                    Some(LiteralKind::Number {
                        base: Base::Decimal,
                        value: 0.0,
                    }),
                ),
            }
        } else {
            self.parse_number_literal(10, Base::Decimal);
        }
    }

    fn parse_number_literal(&mut self, radix: u32, base: Base) {
        if radix != 10 {
            self.advance();
        }
        while self.peek().is_digit(radix) {
            self.advance();
        }
        if radix == 10 && self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let sub = &self.source[self.start..self.crnt];
        let value = if radix == 10 {
            sub.parse::<f32>().unwrap_or(0.0)
        } else {
            if sub.len() > 2 {
                i32::from_str_radix(&sub[2..], radix)
                    .map(|v| v as f32)
                    .unwrap_or(0.0)
            } else {
                0.0
            }
        };
        self.push_token(NumberLit, Some(LiteralKind::Number { base, value }));
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.crnt..].chars().next().unwrap_or('\0');
        self.crnt += c.len_utf8();
        c
    }

    fn push_token(&mut self, token: TokenType, value: Option<LiteralKind>) {
        let lexeme = &self.source[self.start..self.crnt];
        let pos = (self.pos, self.pos + lexeme.chars().count());
        self.pos += lexeme.chars().count();
        self.tokens.push(Token {
            token,
            lexeme: lexeme.to_string(),
            line: self.line,
            value,
            pos,
        });
    }

    fn peek(&self) -> char {
        self.source[self.crnt..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source[self.crnt..].chars().nth(1).unwrap_or('\0')
    }
}

pub fn kwds() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("sh", Sh),
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
        ("enum", Enum),
        ("async", Async),
        ("await", Await),
        ("type", TypeStmt),
        ("pub", Pub),
        ("mut", Mut),
        ("func", Func),
        ("true", TrueLit),
        ("false", FalseLit),
        ("number", NumberIdent),
        ("string", StringIdent),
        ("char", CharIdent),
        ("bool", BoolIdent),
        ("null", Null),
        ("void", VoidIdent),
        ("array", ArrayIdent),
        ("any", AnyIdent),
    ])
}
