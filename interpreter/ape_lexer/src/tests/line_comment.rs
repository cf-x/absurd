use super::*;
/*
tests::line_comments.rs

Unit tests for single line comments:
- // hi
- // hi \n ;
- ; \n // hi;
*/

#[test]
fn test_line_comment_1() {
    let right = get_tokens("// hi ");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test single line comment");
}

#[test]
fn test_line_comment_2() {
    let right = get_tokens("// hi \n ;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 2,
            pos: (2, 3),
            value: None,
        }],
        2,
    );
    assert_eq!(left, right, "test single line comment");
}

#[test]
fn test_line_comment_3() {
    let right = get_tokens("; \n // hi;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 1,
            pos: (1, 2),
            value: None,
        }],
        2,
    );
    assert_eq!(left, right, "test single line comment");
}
