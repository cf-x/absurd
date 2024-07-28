use colored::Colorize;

/// `lexer error (E0x101): unknown character: '{1}', at line {0}`
/// - {0}: usize, line
/// - {1}: char, character
/// example:
///
/// `lexer error (E0x101): unknown character: '💙', at line 32`
pub fn e0x101(line: usize, char: char) {
    throw_err(
        "lexer",
        101,
        format!("unknown character: '{}', at line {}", char, line),
    );
}

/// `lexer error (E0x102): malformed or unterminated char, at line {0}`
/// - {0}: usize, line
/// example:
///
/// `lexer error (E0x102): malformed or unterminated char, at line 5`
pub fn e0x102(line: usize) {
    throw_err(
        "lexer",
        102,
        format!("malformed or unterminated char, at line {}", line),
    );
}

/// `lexer error (E0x103): unterminated string, at line {0}`
/// - {0}: usize, line
/// example:
///
/// `lexer error (E0x103): unterminated string, at line 53`
pub fn e0x103(line: usize) {
    throw_err(
        "lexer",
        103,
        format!("unterminated string, at line {}", line),
    );
}

/// `lexer error (E0x104): failed to parse {1} base number '{2}', at line {0}`
/// - {0}: usize, line
/// - {1}: string, base
/// - {2}: string, substring
/// example:
///
/// `lexer error (E0x104): failed to parse binary base number '0b13', at line 42
pub fn e0x104(line: usize, base: &str, sub: &str) {
    throw_err(
        "lexer",
        104,
        format!(
            "failed to parse {} base number '{}' , at line {}",
            base, sub, line
        ),
    );
}

fn throw_err(kind: &str, code: usize, msg: String) {
    let err_code = format!("E0x{}", code).yellow();
    let head = format!("{} error ({}):", kind, err_code);
    panic!("{} {}", head.red().bold(), msg.red());
}
