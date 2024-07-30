use colored::Colorize;
use ErrorCode::*;
mod msgs;

#[derive(Debug, Clone)]
pub enum ErrorCode {
    /// ## cause
    /// unrecognized character detected by lexer
    ///
    /// ## possible solutions
    /// - remove the character from the source
    /// - add character to the literal types and handle it
    ///
    /// ## message
    /// `lexer error (E0x101): unknown character: '{1}', at line {0}`
    /// - {0}: usize, line
    /// - {1}: char, character
    /// example:
    ///
    /// `lexer error (E0x101): unknown character: '💙', at line 32`
    E0x101,
    /// ## cause
    /// char type isn't closed by `'`, or has invalid charatet inside
    ///
    /// ## possible solutions
    /// - close the character literal by adding `'` at the end
    ///
    /// ## message
    /// `lexer error (E0x102): malformed or unterminated char, at line {0}`
    /// - {0}: usize, line
    /// example:
    ///
    /// `lexer error (E0x102): malformed or unterminated char, at line 5`
    E0x102,
    /// ## cause
    /// string type isn't closed by `"`
    ///
    /// ## possible solutions
    /// - close the string literal by adding `"` at the end
    ///
    /// ## message
    /// `lexer error (E0x103): unterminated string, at line {0}`
    /// - {0}: usize, line
    /// example:
    ///
    /// `lexer error (E0x103): unterminated string, at line 53`
    E0x103,
    /// ## cause
    /// number might contain invalid characters or a digit from the different base
    ///
    /// ## possible solutions
    /// - ensure that number is written in the right base
    ///
    /// ## message
    /// `lexer error (E0x104): failed to parse {1} base number '{2}', at line {0}`
    /// - {0}: usize, line
    /// - {1}: string, base
    /// - {2}: string, substring
    /// example:
    ///
    /// `lexer error (E0x104): failed to parse binary base number '0b13', at line 42
    E0x104,
    /// ## cause
    /// unexpected token found inside the statement
    ///
    /// ## possible solutions
    /// - identify and remove the unexpected token
    ///
    /// ## message
    /// `parser error (E0x201): unexpected token '{1}', at line {0}`
    /// - {0}: line
    /// - {1}: token
    /// example:
    ///
    /// `parser error (E0x201): unexpected token '{', at line 84`
    E0x201,
    /// ## cause
    /// invalid number type
    ///
    /// ## possible solutions
    /// - replace with valid number
    ///
    /// ## message
    /// `parser error (E0x202): failed to unwrap a number '{1}', at line {0}`
    /// - {0}: line
    /// - {1}: number
    /// example:
    ///
    /// `parser error (E0x202): failed to unwrap a number 'iter', at line 12`
    E0x202,
    /// ## cause
    /// invalid tokens, token types or statement/expression structure
    ///
    /// ## possible solutions
    /// - try to match the required structure
    ///
    /// ## message
    /// `parser error (E0x203): failed to parse {1}, at line {0}`
    /// - {0}: line
    /// - {1}: subject
    /// example:
    ///
    /// `parser error (E0x203): failed to parse a block statement, at line 5`
    E0x203,
    /// ## cause
    /// didn't receive expected token
    ///
    /// ## possible solutions
    /// - try to match the required structure
    ///
    /// ## message
    /// `parser error (E0x204): expected a token '{1}', at line {0}`
    /// - {0}: line
    /// - {1}: token
    ///
    /// example:
    ///
    /// `parser error (E0x204): expected a token '{', at line 15`
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

    pub fn throw(&self, code: ErrorCode, line: usize, args: Vec<String>) {
        match code {
            E0x101 => self.e101(line, args),
            E0x102 => self.e102(line, args),
            E0x103 => self.e103(line, args),
            E0x104 => self.e104(line, args),
            E0x201 => self.e201(line, args),
            E0x202 => self.e202(line, args),
            E0x203 => self.e203(line, args),
            E0x204 => self.e204(line, args),
        };
    }

    pub fn print_lines(&self, line: usize) {
        let lines: Vec<&str> = self.source.lines().collect();

        if line > 1 {
            eprintln!(
                "{} | {}",
                (line - 1).to_string().yellow(),
                &lines[line - 2].red()
            );
        }
        eprintln!(
            "{} | {}",
            line.to_string().yellow(),
            &lines[line - 1].red().bold()
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
