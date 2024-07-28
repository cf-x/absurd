use colored::Colorize;

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
pub fn e0x101(line: usize, char: char) {
    throw_err(
        "lexer",
        101,
        format!("unknown character: '{}', at line {}", char, line),
    );
}

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
pub fn e0x102(line: usize) {
    throw_err(
        "lexer",
        102,
        format!("malformed or unterminated char, at line {}", line),
    );
}

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
pub fn e0x103(line: usize) {
    throw_err(
        "lexer",
        103,
        format!("unterminated string, at line {}", line),
    );
}

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
pub fn e0x104(line: usize, base: &str, sub: &str) {
    throw_err(
        "lexer",
        104,
        format!(
            "failed to parse {} base number '{}', at line {}",
            base, sub, line
        ),
    );
}

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
pub fn e0x201(line: usize, token: String) {
    throw_err(
        "parser",
        201,
        format!("unexpected token '{}', at line {}", token, line),
    );
}

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
pub fn e0x202(line: usize, number: String) {
    throw_err(
        "parser",
        202,
        format!("failed to unwrap a number '{}', at line {}", number, line),
    );
}

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
pub fn e0x203(line: usize, subject: String) {
    throw_err(
        "parser",
        203,
        format!("failed to parse {}, at line {}", subject, line),
    );
}

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
pub fn e0x204(line: usize, token: String) {
    throw_err(
        "parser",
        204,
        format!("expected a token '{}', at line {}", token, line),
    );
}

fn throw_err(kind: &str, code: usize, msg: String) {
    let err_code = format!("E0x{}", code).yellow();
    let head = format!("{} error ({}):", kind, err_code);
    panic!("{} {}", head.red().bold(), msg.red());
}
