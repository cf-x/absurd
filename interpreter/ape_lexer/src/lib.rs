use std::collections::HashMap;
pub(crate) mod lexer;

use ape_ast::{
    Token,
    TokenType::{self, *},
};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            keywords: HashMap::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn hashmap_keywords() -> HashMap<&'static str, TokenType> {
        HashMap::from([
            ("let", Let),
            ("if", If),
            ("else", Else),
            ("else if", ElseIf),
            ("return", Return),
            ("while", While),
            ("loop", Loop),
            ("break", Break),
            ("match", Match),
            ("mod", Mod),
            ("use", Use),
            ("as", As),
            ("from", From),
            ("struct", Struct),
            ("impl", Impl),
            ("enum", Enum),
            ("async", Async),
            ("await", Await),
            ("pub", Pub),
            ("mut", Mut),
            ("func", Func),
            ("number", NumberIdent),
            ("string", StringLit),
            ("char", CharLit),
            ("bool", BoolIdent),
            ("null", NullIdent),
            ("void", VoidIdent),
            ("any", AnyIdent),
            ("array", ArrayIdent),
        ])
    }

    fn add_token(&mut self, token: TokenType) {}
}
