use super::*;
/*
tests::char_lit.rs

Unit tests for character literals:
- 'c'
*/

#[test]
fn char_lit_1() {
    let right = get_tokens("'c'");
    let left = get_token(
        vec![Token {
            token: CharLit,
            lexeme: "'c'".to_string(),
            line: 1,
            pos: (1, 4),
            value: Some(LiteralKind::Char { value: 'c' }),
        }],
        1,
    );
    assert_eq!(left, right, "test string literal token: `'c'`");
}
