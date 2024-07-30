mod binary;
mod block_comment;
mod char_lit;
mod chars_len1;
mod chars_len2;
mod float;
mod hexa;
mod ident;
mod integer;
mod kwds;
mod line_comment;
mod octal;
mod string_lit;
use super::*;

pub fn get_tokens(source: &'static str) -> Vec<Token> {
    let err = Error::new(source);
    let mut lexer = Lexer::new(source.to_string(), err);
    lexer.lex()
}

pub fn get_token(tokens: Vec<Token>, line: usize) -> Vec<Token> {
    let mut tokens = tokens;
    tokens.push(Token {
        token: Eof,
        pos: (0, 0),
        lexeme: "\0".to_string(),
        value: None,
        line,
    });
    tokens
}
