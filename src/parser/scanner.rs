// Absurd Scanner, collects tokens from the source.
use crate::ast::{
    Base, LiteralKind, Token,
    TokenType::{self, *},
};
use crate::errors::{Error, ErrorCode::*};
use coloredpp::Colorize;
use std::collections::HashMap;
use unicode_xid::UnicodeXID;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    src: &'a str,
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
    pub fn new(src: &'a str, err: Error, log: bool) -> Self {
        Self {
            src,
            err,
            tokens: vec![],
            kwds: HashMap::from([
                ("sh", Sh),
                ("let", Let),
                ("if", If),
                ("else", Else),
                ("elif", Elif),
                ("return", Return),
                ("while", While),
                ("loop", Loop),
                ("break", Break),
                ("match", Match),
                ("mod", Mod),
                ("use", Use),
                ("as", As),
                ("from", From),
                ("async", Async),
                ("await", Await),
                ("type", TypeStmt),
                ("pub", Pub),
                ("mut", Mut),
                ("func", Func),
                ("true", TrueLit),
                ("false", FalseLit),
                ("number", NumIdent),
                ("string", StrIdent),
                ("char", CharIdent),
                ("bool", BoolIdent),
                ("null", Null),
                ("void", VoidIdent),
                ("array", ArrayIdent),
                ("any", AnyIdent),
            ]),
            line: 1,
            pos: 1,
            start: 0,
            crnt: 0,
            log,
        }
    }

    /// main scanner function
    pub fn scan(&mut self) -> &Vec<Token> {
        if self.log {
            println!("  {}", "collecting tokens...".yellow())
        }
        // advance until the end of the file
        while !self.is_eof() {
            self.start = self.crnt;
            self.advance_token();
        }
        // push 'end of file' token, after reaching it
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
        // return collected tokens
        &self.tokens
    }

    /// checks if end of file is reached
    fn is_eof(&self) -> bool {
        self.crnt >= self.src.len()
    }

    fn advance_token(&mut self) {
        let c = self.advance();
        match c {
            // single character tokens
            '_' => self.push(Underscore, None),
            '%' => self.push(Prcnt, None),
            '(' => self.push(LParen, None),
            ')' => self.push(RParen, None),
            '{' => self.push(LBrace, None),
            '}' => self.push(RBrace, None),
            '[' => self.push(LBracket, None),
            ']' => self.push(RBracket, None),
            ';' => self.push(Semi, None),
            ',' => self.push(Comma, None),
            '?' => self.push(Qstn, None),
            // double character tokens
            ':' => self.dbl_char(':', Colon, DblColon),
            '!' => self.mult_char(Bang, &[('=', BangEq), ('!', DblBang)]),
            '&' => self.dbl_char('&', And, DblAnd),
            '+' => self.mult_char(Plus, &[('+', Incr), ('=', PlusEq)]),
            '-' => self.mult_char(Min, &[('>', Arrow), ('-', Decr), ('=', MinEq)]),
            '*' => self.mult_char(Mul, &[('=', MulEq), ('*', Sqr)]),
            '=' => self.mult_char(Assign, &[('=', Eq), ('>', ArrowBig)]),
            '|' => self.dbl_char('|', Pipe, Or),
            '.' => self.dbl_char('.', Dot, DblDot),
            '<' => self.dbl_char('=', Ls, LsOrEq),
            '>' => self.dbl_char('=', Gr, GrOrEq),
            '\\' => self.mult_char(Esc, &[('{', LParse), ('}', RParse)]),
            // whitespaces and comments
            '/' => self.div(),
            '#' => self.line_comment(),
            '\r' => {}
            '\t' => self.pos += 4,
            ' ' => self.pos += 1,
            '\n' => {
                self.pos = 1;
                self.line += 1;
            }
            // literals and identifiers
            '\'' => self.charlit(),
            '"' => self.strlit(),
            c if c.is_ascii_digit() => self.numlit(c),
            c if UnicodeXID::is_xid_start(c) || c == '_' => self.ident(),
            _ => self.push(Ident, None),
        };
    }

    /// function for handling two character tokens with multiple variations
    fn mult_char(&mut self, single: TokenType, variants: &[(char, TokenType)]) {
        let token_type =
            if let Some(&(_, ref token)) = variants.iter().find(|&&(ch, _)| ch == self.peek()) {
                self.advance();
                token.clone()
            } else {
                single
            };
        self.push(token_type, None);
    }

    /// function for handling two character tokens with a single variation
    fn dbl_char(&mut self, next_char: char, single: TokenType, double: TokenType) {
        let token_type = if self.peek() == next_char {
            self.advance();
            double
        } else {
            single
        };
        self.push(token_type, None);
    }

    /// `/`, `//`, `/*`, `/=`
    fn div(&mut self) {
        match self.peek() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            '=' => {
                self.advance();
                self.push(DivEq, None);
            }
            _ => self.push(Div, None),
        }
    }

    /// ignores characters in comment lines
    /// `//`, `#`
    fn line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_eof() {
            self.advance();
        }
        self.pos = 1;
    }

    /// ignores characters in comment blocks
    /// `/*`, `*/`
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

    /// handles 'c'haracters
    /// @todo handle escape characters and UNICODE code characters
    fn charlit(&mut self) {
        let value = if self.peek() != '\'' && !self.is_eof() {
            self.advance()
        } else {
            self.err
                .throw(E0x101, self.line, (self.pos - 1, self.pos), vec![]);
            return;
        };
        if self.peek() != '\'' {
            self.err
                .throw(E0x101, self.line, (self.pos - 1, self.pos), vec![]);
            return;
        }
        self.advance();
        self.push(CharLit, Some(LiteralKind::Char { value }));
    }

    /// handles "strings"
    /// @todo handle escaping and parsing
    fn strlit(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
                self.pos = 1;
            }
            self.advance();
        }
        if self.is_eof() {
            self.err
                .throw(E0x102, self.line, (self.pos - 1, self.pos), vec![]);
            return;
        }
        self.advance();
        let value: String = self.src[self.start + 1..self.crnt - 1].to_string();
        self.push(StrLit, Some(LiteralKind::String { value }));
    }

    /// handles identifier, Unicode emoji, '_' or alphanumeric
    fn ident(&mut self) {
        while UnicodeXID::is_xid_continue(self.peek()) || self.peek() == '_' {
            self.advance();
        }
        let sub = &self.src[self.start..self.crnt];
        let token = self.kwds.get(sub).cloned().unwrap_or(Ident);
        match token {
            TrueLit => self.push(TrueLit, Some(LiteralKind::Bool { value: true })),
            FalseLit => self.push(FalseLit, Some(LiteralKind::Bool { value: false })),
            _ => self.push(token, None),
        }
    }

    /// handles numbers and supports multiple base systems
    fn numlit(&mut self, c: char) {
        if c == '0' {
            match self.peek() {
                'b' => self.parse_numlit(2, Base::Binary),
                'o' => self.parse_numlit(8, Base::Octal),
                'x' => self.parse_numlit(16, Base::Hexadecimal),
                '0'..='9' | '_' | '.' => self.parse_numlit(10, Base::Decimal),
                _ => self.push(
                    NumLit,
                    Some(LiteralKind::Number {
                        base: Base::Decimal,
                        value: 0.0,
                    }),
                ),
            }
        } else {
            self.parse_numlit(10, Base::Decimal);
        }
    }

    /// parses numbers in the number literal token
    fn parse_numlit(&mut self, radix: u32, base: Base) {
        if radix != 10 {
            self.advance();
        }

        while self.peek().is_digit(radix) || self.peek() == '_' {
            if self.peek() == '_' {
                self.advance();
            } else {
                self.advance();
            }
        }

        if radix == 10 && self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) || self.peek() == '_' {
                if self.peek() == '_' {
                    self.advance();
                } else {
                    self.advance();
                }
            }
        }

        let sub: String = self.src[self.start..self.crnt]
            .chars()
            .filter(|&c| c != '_')
            .collect();
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
        self.push(NumLit, Some(LiteralKind::Number { base, value }));
    }

    /// just as name says, pushes tokens
    fn push(&mut self, token: TokenType, value: Option<LiteralKind>) {
        let lexeme = &self.src[self.start..self.crnt];
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

    /// checks the next character
    fn peek(&self) -> char {
        self.src[self.crnt..].chars().next().unwrap_or('\0')
    }

    /// checks the next of the next character
    fn peek_next(&self) -> char {
        self.src[self.crnt..].chars().nth(1).unwrap_or('\0')
    }

    /// advances source by 1 character
    fn advance(&mut self) -> char {
        let c = self.src[self.crnt..].chars().next().unwrap_or('\0');
        self.crnt += c.len_utf8();
        c
    }
}
