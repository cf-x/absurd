use super::*;

/// tests if new tokens are created normally
#[test]
fn test_new_token() {
    let left = Token {
        token: TokenType::And,
        len: 1,
        lexeme: "&".to_string(),
        value: None,
        line: 1,
    };
    let right = Token::new(TokenType::And, 1, "&".to_string(), None, 1);
    assert_eq!(left, right, "tested new '&' token");
}
