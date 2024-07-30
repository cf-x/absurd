use super::*;
/*
tests::binary.rs

Unit tests for binary number literals:
- 0b11
- 0b0101
- 0b111
*/

#[test]
fn test_binary_1() {
    let right = get_tokens("0b11");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0b11".to_string(),
            line: 1,
            pos: (1, 5),
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
            pos: (1, 7),
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
            pos: (1, 6),
            value: Some(LiteralKind::Number {
                base: Base::Binary,
                value: 7.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test binary token: `0b111`");
}
