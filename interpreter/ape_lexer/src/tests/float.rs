use super::*;

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
