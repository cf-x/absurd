use super::*;
/*
tests::kwds.rs

Unit tests for keywords:
- func
- as
- impl
*/

#[test]
fn test_kwds_1() {
    let right = get_tokens("func");
    let left = get_token(
        vec![Token {
            token: Func,
            lexeme: "func".to_string(),
            line: 1,
            pos: (1, 5),
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
            pos: (1, 3),
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
            pos: (1, 5),
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test keyword token: `impl`");
}
