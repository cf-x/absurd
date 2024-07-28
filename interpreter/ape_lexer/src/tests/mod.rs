mod char_lit;
mod string_lit;
mod block_comment;
mod line_comment;
mod chars_len1;
mod chars_len2;
mod kwds;
mod integer;
mod float;
mod binary;
mod octal;
mod hexa;
mod ident;
use super::*;

pub fn get_tokens(source: &'static str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string());
    lexer.lex()
}

pub fn get_token(tokens: Vec<Token>, line: usize) -> Vec<Token> {
    let mut tokens = tokens;
    tokens.push(Token {
        token: Eof,
        len: 0,
        lexeme: "\0".to_string(),
        value: None,
        line,
    });
    tokens
}
