use std::process::exit;

use colored::Colorize;
use ErrorCode::*;
mod msgs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ErrorCode {
    /// ## message
    /// `lexer error (E0x101): unknown character: '{0}'`
    /// - {0}: char, character
    /// example:
    ///
    /// `lexer error (E0x101): unknown character: '💙', at line 32`
    E0x101,
    /// ## message
    /// `lexer error (E0x102): malformed or unterminated char`
    /// example:
    ///
    /// `lexer error (E0x102): malformed or unterminated char`
    E0x102,
    /// ## message
    /// `lexer error (E0x103): unterminated string`
    /// example:
    ///
    /// `lexer error (E0x103): unterminated string`
    E0x103,
    /// ## message
    /// `lexer error (E0x104): failed to parse {1} base number '{2}'`
    /// - {0}: string, base
    /// - {1}: string, substring
    /// example:
    ///
    /// `lexer error (E0x104): failed to parse binary base number '0b13'
    E0x104,
    /// ## message
    /// `parser error (E0x201): unexpected token '{0}'`
    /// - {0}: token
    /// example:
    ///
    /// `parser error (E0x201): unexpected token '{'`
    E0x201,
    /// ## message
    /// `parser error (E0x202): failed to unwrap a number '{0}'`
    /// - {0}: line
    /// - {1}: number
    /// example:
    ///
    /// `parser error (E0x202): failed to unwrap a number 'iter'`
    E0x202,
    /// ## message
    /// `parser error (E0x203): failed to parse {0}`
    /// - {0}: subject
    /// example:
    ///
    /// `parser error (E0x203): failed to parse a block statement`
    E0x203,
    /// ## message
    /// `parser error (E0x204): expected a token '{0}'`
    /// - {0}: token
    /// example:
    ///
    /// `parser error (E0x204): expected a token '{'`
    E0x204,
    /// ## message
    /// `runtime error (E0x301): type mismatch: expected '{0}', got '{1}'`
    /// - {0}: expected type
    /// - {1}: actual type
    /// example:
    ///
    /// `runtime error (E0x301): type mismatch: expected 'number', got 'string'`
    E0x301,
    /// ## message
    /// `runtime error (E0x302): break statement not within a loop`
    E0x302,
    /// ## message
    /// `runtime error (E0x303): return statement not within a function`
    E0x303,
    /// ## message
    /// `runtime error (E0x304): await statement not within an async function`
    E0x304,
    /// ## message
    /// `runtime error (E0x305): invalid function return type`
    E0x305,
    /// ## message
    /// `runtime error (E0x306): failed to resolve {0}`
    /// - {0}: subject
    /// example:
    ///
    /// `runtime error (E0x306): failed to resolve a block statement`
    E0x306,
    /// ## message
    /// `runtime error (E0x307): '{0}' is already declared`
    /// - {0}: subject
    /// example:
    ///
    /// `runtime error (E0x307): 'x' is already declared`
    E0x307,
    /// ## message
    /// `runtime error (E0x308): stack underflow`
    E0x308,
    /// ## message
    /// `runtime error (E0x309): stack overflow`
    E0x309,
    /// ## message
    /// `runtime error (E0x401): function must have one name`
    E0x401,
    /// ## message
    /// `runtime error (E0x402): public variable must have a value`
    E0x402,
    /// ## message
    /// `runtime error (E0x403): invalid function body`
    E0x403,
    /// ## message
    /// `runtime error (E0x404): failed to create a function`
    E0x404,
    /// ## message
    /// `runtime error (E0x405): invalid number of arguments`
    E0x405,
    /// ## message
    /// `runtime error (E0x406): missing return statement`
    E0x406,
}

#[derive(Debug, Clone)]
pub struct Error {
    source: String,
}

impl Error {
    pub fn new(src: &str) -> Self {
        Error {
            source: src.to_string(),
        }
    }

    pub fn throw(&self, code: ErrorCode, line: usize, pos: (usize, usize), args: Vec<String>) {
        match code {
            E0x101 => self.e101(line, pos, args),
            E0x102 => self.e102(line, pos, args),
            E0x103 => self.e103(line, pos, args),
            E0x104 => self.e104(line, pos, args),
            E0x201 => self.e201(line, pos, args),
            E0x202 => self.e202(line, pos, args),
            E0x203 => self.e203(line, pos, args),
            E0x204 => self.e204(line, pos, args),
            E0x301 => self.e301(line, pos, args),
            E0x302 => self.e302(line, pos),
            E0x303 => self.e303(line, pos),
            E0x304 => self.e304(line, pos),
            E0x305 => self.e305(line, pos),
            E0x306 => self.e306(line, pos, args),
            E0x307 => self.e307(line, pos, args),
            E0x308 => self.e308(line, pos),
            E0x309 => self.e309(line, pos),
            E0x401 => self.e401(line, pos),
            E0x402 => self.e402(line, pos),
            E0x403 => self.e403(line, pos),
            E0x404 => self.e404(line, pos),
            E0x405 => self.e405(line, pos),
            E0x406 => self.e406(line, pos),
        };
    }

    pub fn print_lines(&self, line: usize, pos: (usize, usize)) {
        let lines: Vec<&str> = self.source.lines().collect();

        if line > 1 {
            eprintln!(
                "{} | {}",
                (line - 1).to_string().yellow(),
                &lines[line - 2].red()
            );
        }

        let before = &lines[line - 1][..pos.0 - 1];
        let to_underscore = &lines[line - 1][pos.0 - 1..pos.1 - 1];
        let after = &lines[line - 1][pos.1 - 1..];

        eprintln!(
            "{} | {}{}{}",
            line.to_string().yellow(),
            before.red().bold(),
            to_underscore.red().bold().underline(),
            after.red().bold()
        );

        if line < lines.len() {
            eprintln!(
                "{} | {}",
                (line + 1).to_string().yellow(),
                &lines[line].red()
            );
        }
    }

    pub fn panic(&self, kind: &str, code: usize, msg: String) {
        let err_code = format!("E0x{}", code).yellow();
        let head = format!("{} error ({}):", kind, err_code);
        eprintln!("{} {}", head.red().bold(), msg.red());
        exit(0);
    }

    pub fn eprintln(&self, kind: &str, code: usize, msg: String) {
        let err_code = format!("E0x{}", code).yellow();
        let head = format!("{} error ({}):", kind, err_code);
        eprintln!("{} {}", head.red().bold(), msg.red());
    }
    #[allow(dead_code)]
    pub fn warn(&self, msg: String) {
        let head = "warning:".bold().yellow();
        eprintln!("{} {}", head, msg.yellow());
    }
}
