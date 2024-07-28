use super::*;

#[test]
fn test_ident_1() {
    let right = get_tokens("name");
    let left = get_token(
        vec![Token {
            token: Ident,
            lexeme: "name".to_string(),
            line: 1,
            len: 4,
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
            len: 5,
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
            len: 5,
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test identifier token: `name1`");
}
