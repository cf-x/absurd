use super::*;

#[test]
fn test_string_lit_1() {
    let right = get_tokens("\"hi\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"hi\"".to_string(),
            line: 1,
            len: 4,
            value: Some(LiteralKind::String {
                value: "hi".to_string(),
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `\"hi\"`");
}

#[test]
fn test_string_lit_2() {
    let right = get_tokens("\"3 2\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"3 2\"".to_string(),
            line: 1,
            len: 5,
            value: Some(LiteralKind::String {
                value: "3 2".to_string(),
            }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `\"3 2\"`");
}

#[test]
fn test_string_lit_3() {
    let right = get_tokens("\"h\ni\"");
    let left = get_token(
        vec![Token {
            token: StringLit,
            lexeme: "\"h\ni\"".to_string(),
            line: 2,
            len: 5,
            value: Some(LiteralKind::String {
                value: "h\ni".to_string(),
            }),
        }],
        2,
    );
    assert_eq!(left, right, "test string literal token: `\"h\ni\"`");
}
