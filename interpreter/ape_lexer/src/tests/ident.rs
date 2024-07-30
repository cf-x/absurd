use super::*;
/*
tests::ident.rs

Unit tests for identifiers:
- name
- na_me
- name1
*/
#[test]
fn test_ident_1() {
    let right = get_tokens("name");
    let left = get_token(
        vec![Token {
            token: Ident,
            lexeme: "name".to_string(),
            line: 1,
            pos: (1, 5),
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
            pos: (1, 6),
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
            pos: (1, 6),
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test identifier token: `name1`");
}
