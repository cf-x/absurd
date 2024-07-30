use super::*;
/*
tests::hexa.rs

Unit tests for hexadecimal number literals:
- "0xf"
- "0xff2"
- "0x00f"
*/

#[test]
fn test_hexadecimal_1() {
    let right = get_tokens("0xf");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "0xf".to_string(),
            line: 1,
            pos: (1, 4),
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
            pos: (1, 6),
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
            pos: (1, 6),
            value: Some(LiteralKind::Number {
                base: Base::Hexadecimal,
                value: 15.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test hexadecimal token: `0x00f`");
}
