use super::*;

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
