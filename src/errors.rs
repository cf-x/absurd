// handles Absurd errors
use super::manifest::Project;
use coloredpp::Colorize;
use std::process::exit;
use ErrorCode::*;

#[derive(Debug, Clone)]
pub enum ErrorCode {
    /// `syntax error (E0x101): malformed or unterminated char`
    E0x101,
    /// `syntax error (E0x102): unterminated string`
    E0x102,
    /// `syntax error (E0x103): unexpected token '{0}'`
    /// - {0}: token
    E0x103,
    /// `syntax error (E0x104): failed to unwrap a number '{0}'`
    /// - {0}: line
    /// - {1}: number
    E0x104,
    /// `syntax error (E0x105): failed to parse {0}`
    /// - {0}: subject
    E0x105,
    /// `syntax error (E0x106): expected a token '{0}'`
    /// - {0}: token
    E0x106,
    /// `sybtax error (E0x107): invalid assignment target`
    E0x107,
    /// `runtime error (E0x301): type mismatch: expected '{0}', got '{1}'`
    /// - {0}: expected type
    /// - {1}: actual type
    E0x301,
    /// `runtime error (E0x302): break statement not within a loop`
    E0x302,
    /// `runtime error (E0x303): return statement not within a function`
    E0x303,
    /// `runtime error (E0x304): failed to resolve {0}`
    /// - {0}: subject
    E0x304,
    /// `runtime error (E0x305): '{0}' is already declared`
    /// - {0}: subject
    E0x305,
    /// `runtime error (E0x306): stack underflow`
    E0x306,
    /// `runtime error (E0x401): function must have one name`
    E0x401,
    /// `runtime error (E0x402): public variable must have a value`
    E0x402,
    /// `runtime error (E0x404): failed to create a function`
    E0x404,
    /// `runtime error (E0x405): invalid number of arguments`
    E0x405,
    /// `runtime error (E0x406): missing return statement`
    E0x406,
    /// `runtime error (E0x410): can not assign to an immutable variable`
    E0x410,
    /// `runtime error (E0x411): can not assign to a public variable`
    E0x411,
    /// `runtime error (E0x412): invalid type, while assigning to a variable '{0}'`
    /// - {0}: variable
    E0x412,
    /// `runtime error (E0x413): can not assign to a non-variable`
    E0x413,
    /// `runtime error (E0x414): failed to assign a value`
    E0x414,
    /// `runtime error (E0x415): side effects are disabled`
    E0x415,
    /// `runtime error (E0x416): failed to get values from {0}`
    /// - {0}: source
    E0x416,
    /// `environment error (E0x501): failed to get a distance`
    E0x501,
    /// `environment error (E0x502): failed to resolve a value`
    E0x502,
}

#[derive(Debug, Clone)]
pub struct Error {
    source: String,
    project: Project,
}

impl Error {
    pub fn new(src: &str, project: Project) -> Self {
        Error {
            source: src.to_string(),
            project,
        }
    }

    pub fn throw(&self, code: ErrorCode, line: usize, pos: (usize, usize), args: Vec<String>) {
        match code {
            E0x101 => self.error(
                101,
                "syntax",
                "malformed or unterminated char".to_string(),
                line,
                pos,
            ),
            E0x102 => self.error(102, "syntax", "unterminated string".to_string(), line, pos),
            E0x103 => self.error(
                103,
                "syntax",
                format!("unexpected token '{}'", args[0]),
                line,
                pos,
            ),
            E0x104 => self.error(
                104,
                "syntax",
                format!("failed to unwrap a number '{}'", args[0]),
                line,
                pos,
            ),
            E0x105 => self.error(
                105,
                "syntax",
                format!("failed to parse '{}'", args[0]),
                line,
                pos,
            ),
            E0x106 => self.error(
                106,
                "syntax",
                format!("expected a token '{}'", args[0]),
                line,
                pos,
            ),
            E0x107 => self.error(
                107,
                "syntax",
                format!("invalid assignment target"),
                line,
                pos,
            ),
            E0x301 => self.error(
                301,
                "runtime",
                format!("type mismatch: expected '{}', got '{}'", args[0], args[1]),
                line,
                pos,
            ),
            E0x302 => self.error(
                302,
                "runtime",
                "break statement not within a loop".to_string(),
                line,
                pos,
            ),
            E0x303 => self.error(
                303,
                "runtime",
                "return statement not within a function".to_string(),
                line,
                pos,
            ),
            E0x304 => self.error(
                306,
                "runtime",
                format!("failed to resolve '{}'", args[0]),
                line,
                pos,
            ),
            E0x305 => self.error(
                307,
                "runtime",
                format!("'{}' is already declared", args[0]),
                line,
                pos,
            ),
            E0x306 => self.error(308, "runtime", "stack underflow".to_string(), line, pos),
            E0x401 => self.error(
                401,
                "runtime",
                "function must have one name".to_string(),
                line,
                pos,
            ),
            E0x402 => self.error(
                402,
                "runtime",
                "public variable must have a value".to_string(),
                line,
                pos,
            ),

            E0x404 => self.error(
                404,
                "runtime",
                "failed to create a function".to_string(),
                line,
                pos,
            ),
            E0x405 => self.error(
                405,
                "runtime",
                "invalid number of arguments".to_string(),
                line,
                pos,
            ),
            E0x406 => self.error(
                406,
                "runtime",
                "missing return statement".to_string(),
                line,
                pos,
            ),
            E0x410 => self.error(
                410,
                "runtime",
                "can not assign to an immutable variable".to_string(),
                line,
                pos,
            ),
            E0x411 => self.error(
                411,
                "runtime",
                "can not assign to a public variable".to_string(),
                line,
                pos,
            ),
            E0x412 => self.error(
                412,
                "runtime",
                format!("invalid type, while assigning to a variable '{}'", args[0]),
                line,
                pos,
            ),
            E0x413 => self.error(
                413,
                "runtime",
                "can not assign to a non-variable".to_string(),
                line,
                pos,
            ),
            E0x414 => self.error(
                414,
                "runtime",
                "failed to assign a value".to_string(),
                line,
                pos,
            ),
            E0x415 => self.error(
                415,
                "runtime",
                "side effects are disabled".to_string(),
                line,
                pos,
            ),
            E0x416 => self.error(
                416,
                "runtime",
                format!("failed to get values from {}", args[0]),
                line,
                pos,
            ),
            E0x501 => self.error(
                501,
                "environment",
                "failed to get a distance".to_string(),
                line,
                pos,
            ),
            E0x502 => self.error(
                502,
                "environment",
                "failed to resolve a value".to_string(),
                line,
                pos,
            ),
        };
    }
}

impl Error {
    pub fn error(&self, code: usize, head: &str, msg: String, line: usize, pos: (usize, usize)) {
        let mut is_snippet = false;
        if line != 0 || pos != (0, 0) {
            is_snippet = true;
            self.print_lines(line, pos)
        }
        let msg = match is_snippet {
            true => format!("{}, at line {}:{}-{}", msg, line, pos.0, pos.1),
            false => msg,
        };
        self.panic(head, code, msg)
    }

    pub fn print_lines(&self, line: usize, pos: (usize, usize)) {
        let lines: Vec<&str> = self.source.lines().collect();

        let snippet = self.project.snippet as isize;

        if snippet < 0 {
            return;
        }

        let start = if line as isize > snippet {
            line - snippet as usize
        } else {
            1
        };
        let end = if line + snippet as usize <= lines.len() {
            line + snippet as usize
        } else {
            lines.len()
        };

        for i in start..=end {
            if i == line {
                let line_content = lines[i - 1];
                let (before, to_underscore, after) =
                    self.split_line_at_char_indices(line_content, pos);

                eprintln!(
                    "{} | {}{}{}",
                    i.to_string().yellow(),
                    before.red().bold(),
                    to_underscore.red().bold().underline(),
                    after.red().bold()
                );
            } else {
                eprintln!("{} | {}", i.to_string().yellow(), &lines[i - 1].red());
            }
        }
    }

    pub fn panic(&self, kind: &str, code: usize, msg: String) {
        let err_code = format!("E0x{}", code).yellow();
        let head = format!("{} error {}:", kind, err_code);
        eprintln!(
            "{} {}",
            head.fg_hex("#fc4949").bold(),
            msg.fg_hex("#fc4949")
        );
        exit(0);
    }

    fn split_line_at_char_indices(
        &self,
        line: &str,
        pos: (usize, usize),
    ) -> (String, String, String) {
        let mut chars = line.chars();
        let before: String = if pos.0 > 0 {
            chars.by_ref().take(pos.0 - 1).collect()
        } else {
            String::new()
        };
        let to_underscore: String = chars.by_ref().take(pos.1 - pos.0).collect();
        let after: String = chars.collect();
        (before, to_underscore, after)
    }
}

pub fn raw(msg: &str) {
    eprintln!("{}", msg.red());
    exit(0);
}
