use super::*;

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
