use colored::Colorize;
use ErrorCode::*;
mod msgs;

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
        panic!("{} {}", head.red().bold(), msg.red());
    }

    pub fn eprintln(&self, kind: &str, code: usize, msg: String) {
        let err_code = format!("E0x{}", code).yellow();
        let head = format!("{} error ({}):", kind, err_code);
        eprintln!("{} {}", head.red().bold(), msg.red());
    }

    pub fn warn(&self, msg: String) {
        let head = "warning:".bold().yellow();
        eprintln!("{} {}", head, msg.yellow());
    }
}
