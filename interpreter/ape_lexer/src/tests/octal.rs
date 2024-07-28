use super::*;

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
