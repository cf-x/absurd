use super::*;
use crate::ast::{TokenType::*, *};
/*
tests::float.rs

Unit tests for floating point number literals:
- "2.0"
- "0.314"
- "2424.442"
todo:
- "2.p3" // periods
- "2.21p1" // periods
*/

#[test]
fn test_float_1() {
    let right = get_tokens("2.0");
    let left = get_token(
        vec![Token {
            token: NumberLit,
            lexeme: "2.0".to_string(),
            line: 1,
            pos: (1, 4),
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
            pos: (1, 6),
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
            pos: (1, 9),
            value: Some(LiteralKind::Number {
                base: Base::Decimal,
                value: 2424.442,
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test floating token: `2424.442`");
}
