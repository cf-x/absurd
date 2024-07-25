use super::*;

fn get_tokens(source: &'static str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string());
    lexer.lex()
}

fn get_token(tokens: Vec<Token>, line: usize) -> Vec<Token> {
    let mut tokens = tokens;
    tokens.push(Token {
        token: Eof,
        len: 0,
        lexeme: "\0".to_string(),
        value: None,
        line,
    });
    tokens
}

#[test]
fn test_char_lit_1() {
    let right = get_tokens("'c'");
    let left = get_token(
        vec![Token {
            token: CharLit,
            lexeme: "'c'".to_string(),
            line: 1,
            len: 3,
            value: Some(LiteralKind::Char { value: 'c' }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `'c'`");
}

#[test]
fn test_string_lit_1() {
    let right = get_tokens("\"hi\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"hi\"".to_string(),
            line: 1,
            len: 4,
            value: Some(LiteralKind::String {
                value: "hi".to_string(),
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `\"hi\"`");
}

#[test]
fn test_string_lit_2() {
    let right = get_tokens("\"3 2\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"3 2\"".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::String {
                value: "3 2".to_string(),
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `\"3 2\"`");
}

#[test]
fn test_string_lit_3() {
    let right = get_tokens("\"h\ni\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"h\ni\"".to_string(),
            line: 2,
            len: 5,
            value: Some(LiteralKind::String {
                value: "h\ni".to_string(),
            }),
        }],
        2,
    );
    assert_eq!(left, right, "test string literal token: `\"h\ni\"`");
}

#[test]
fn test_block_comment_1() {
    let right = get_tokens("/* hi */");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test block comment");
}

#[test]
fn test_block_comment_2() {
    let right = get_tokens("/* hi \n */");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test block comment");
}

#[test]
fn test_block_comment_3() {
    let right = get_tokens("/* hi */ ;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 1,
            len: 1,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test block comment");
}

#[test]
fn test_line_comment_1() {
    let right = get_tokens("// hi ");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test single line comment");
}

#[test]
fn test_line_comment_2() {
    let right = get_tokens("// hi \n ;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 2,
            len: 1,
            value: None,
        }],
        2,
    );
    assert_eq!(left, right, "test single line comment");
}

#[test]
fn test_line_comment_3() {
    let right = get_tokens("; \n // hi;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 1,
            len: 1,
            value: None,
        }],
        2,
    );
    assert_eq!(left, right, "test single line comment");
}

#[test]
fn test_char_len1_1() {
    let right = get_tokens(";");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 1,
            len: 1,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 1 char length token: `;`");
}

#[test]
fn test_char_len1_2() {
    let right = get_tokens("-");
    let left = get_token(
        vec![Token {
            token: Minus,
            lexeme: "-".to_string(),
            line: 1,
            len: 1,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 1 char length token: `-`");
}

#[test]
fn test_char_len1_3() {
    let right = get_tokens("\\");
    let left = get_token(
        vec![Token {
            token: Escape,
            lexeme: "\\".to_string(),
            line: 1,
            len: 1,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 1 char length token: `\\`");
}

#[test]
fn test_char_len2_1() {
    let right = get_tokens("--");
    let left = get_token(
        vec![Token {
            token: Decr,
            lexeme: "--".to_string(),
            line: 1,
            len: 2,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 2 char length token: `--`");
}

#[test]
fn test_char_len2_2() {
    let right = get_tokens(">=");
    let left = get_token(
        vec![Token {
            token: GreaterOrEq,
            lexeme: ">=".to_string(),
            line: 1,
            len: 2,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 2 char length token: `->`");
}

#[test]
fn test_char_len2_3() {
    let right = get_tokens("..");
    let left = get_token(
        vec![Token {
            token: DotDot,
            lexeme: "..".to_string(),
            line: 1,
            len: 2,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test 2 char length token: `..`");
}

#[test]
fn test_kwds_1() {
    let right = get_tokens("func");
    let left = get_token(
        vec![Token {
            token: Func,
            lexeme: "func".to_string(),
            line: 1,
            len: 4,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test keyword token: `func`");
}

#[test]
fn test_kwds_2() {
    let right = get_tokens("as");
    let left = get_token(
        vec![Token {
            token: As,
            lexeme: "as".to_string(),
            line: 1,
            len: 2,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test keyword token: `as`");
}

#[test]
fn test_kwds_3() {
    let right = get_tokens("impl");
    let left = get_token(
        vec![Token {
            token: Impl,
            lexeme: "impl".to_string(),
            line: 1,
            len: 4,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test keyword token: `impl`");
}

#[test]
fn test_integer_1() {
    let right = get_tokens("2");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2".to_string(),
            line: 1,
            len: 1,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test integer token: `2`");
}

#[test]
fn test_integer_2() {
    let right = get_tokens("0");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0".to_string(),
            line: 1,
            len: 1,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 0.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test integer token: `0`");
}

#[test]
fn test_integer_3() {
    let right = get_tokens("2342");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2342".to_string(),
            line: 1,
            len: 4,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2342.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test integer token: `2342`");
}

#[test]
fn test_float_1() {
    let right = get_tokens("2.0");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2.0".to_string(),
            line: 1,
            len: 3,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test floating token: `2.0`");
}

#[test]
fn test_float_2() {
    let right = get_tokens("0.314");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0.314".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 0.314,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test floating token: `0.314`");
}

#[test]
fn test_float_3() {
    let right = get_tokens("2424.442");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2424.442".to_string(),
            line: 1,
            len: 8,
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2424.442,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test floating token: `2424.442`");
}

#[test]
fn test_binary_1() {
    let right = get_tokens("0b11");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0b11".to_string(),
            line: 1,
            len: 4,
            value: Some(LiteralKind::Number {
                base: Base::Binary,
                value: 3.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test binary token: `0b11`");
}

#[test]
fn test_binary_2() {
    let right = get_tokens("0b0101");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0b0101".to_string(),
            line: 1,
            len: 6,
            value: Some(LiteralKind::Number {
                base: Base::Binary,
                value: 5.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test binary token: `0b0101`");
}

#[test]
fn test_binary_3() {
    let right = get_tokens("0b111");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0b111".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::Number {
                base: Base::Binary,
                value: 7.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test binary token: `0b111`");
}

#[test]
fn test_octal_1() {
    let right = get_tokens("0o7");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0o7".to_string(),
            line: 1,
            len: 3,
            value: Some(LiteralKind::Number {
                base: Base::Octal,
                value: 7.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test octal token: `0o7`");
}

#[test]
fn test_octal_2() {
    let right = get_tokens("0o27");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0o27".to_string(),
            line: 1,
            len: 4,
            value: Some(LiteralKind::Number {
                base: Base::Octal,
                value: 23.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test octal token: `0o27`");
}

#[test]
fn test_octal_3() {
    let right = get_tokens("0o0027");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0o0027".to_string(),
            line: 1,
            len: 6,
            value: Some(LiteralKind::Number {
                base: Base::Octal,
                value: 23.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test octal token: `0o0027`");
}

#[test]
fn test_hexadecimal_1() {
    let right = get_tokens("0xf");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0xf".to_string(),
            line: 1,
            len: 3,
            value: Some(LiteralKind::Number {
                base: Base::Hexadecimal,
                value: 15.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test hexadecimal token: `0xf`");
}

#[test]
fn test_hexademical_2() {
    let right = get_tokens("0xff2");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0xff2".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::Number {
                base: Base::Hexadecimal,
                value: 4082.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test hexadecimal token: `0xff2`");
}

#[test]
fn test_hexademical_3() {
    let right = get_tokens("0x00f");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0x00f".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::Number {
                base: Base::Hexadecimal,
                value: 15.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test hexadecimal token: `0x00f`");
}

#[test]
fn test_ident_1() {
    let right = get_tokens("name");
    let left = get_token(
        vec![Token {
            token: Ident,
            lexeme: "name".to_string(),
            line: 1,
            len: 4,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test identifier token: `name`");
}

#[test]
fn test_ident_2() {
    let right = get_tokens("na_me");
    let left = get_token(
        vec![Token {
            token: Ident,
            lexeme: "na_me".to_string(),
            line: 1,
            len: 5,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test identifier token: `na_me`");
}

#[test]
fn test_ident_3() {
    let right = get_tokens("name1");
    let left = get_token(
        vec![Token {
            token: Ident,
            lexeme: "name1".to_string(),
            line: 1,
            len: 5,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test identifier token: `name1`");
}
