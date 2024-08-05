use super::*;
use crate::ast::{TokenType::*, *};
/*
tests::block_comment.rs

Unit tests for block comments:
- /* hi */
- /* hi \n */
- /* hi */ ;
*/

#[test]
fn test_block_comment_1() {
    let right = get_tokens("/* hi */");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test block comment");
}

#[test]
fn test_block_comment_2() {
    let right = get_tokens("/* hi \n */");
    let left = get_token(vec![], 1);
    assert_eq!(left, right, "test block comment");
}

#[test]
fn test_block_comment_3() {
    let right = get_tokens("/* hi */ ;");
    let left = get_token(
        vec![Token {
            token: Semi,
            lexeme: ";".to_string(),
            line: 1,
            pos: (9, 10),
            value: None,
        }],
        1,
    );
    assert_eq!(left, right, "test block comment");
}
