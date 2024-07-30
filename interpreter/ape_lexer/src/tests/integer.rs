use super::*;
/*
tests::integer.rs

Unit tests for integer number literals:
- 2
- 0
- 2342
todo:
- 3_342 // separators for better readiblity
- 44e6 // exponentials
*/

#[test]
fn test_integer_1() {
    let right = get_tokens("2");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2".to_string(),
            line: 1,
            pos: (1, 2),
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
            pos: (1, 2),
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
            pos: (1, 5),
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2342.0,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test integer token: `2342`");
}
