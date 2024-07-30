use ape_ast::{Statement, Token};
use ape_errors::Error;
use ape_lexer::Lexer;
use ape_parser::Parser;

#[derive(Debug, Clone)]
pub struct Bundler {
    err: Error,
}

impl Bundler {
    pub fn new(src: &str) -> Self {
        Bundler {
            err: Error::new(src),
        }
    }
    pub fn lexer(&self, src: &str) -> Vec<Token> {
        let mut lex = Lexer::new(src.to_string(), self.err.clone());
        lex.lex()
    }
    pub fn parser(&self, tokens: Vec<Token>) -> Vec<Statement> {
        let mut parser = Parser::new(tokens, self.err.clone());
        parser.parse()
    }
    pub fn front(&self, src: &str) -> Vec<Statement> {
        let tokens = self.lexer(src);
        self.parser(tokens)
    }
    /*
        @todo:
        - analyzer
        - front_full (lexer + parser + analyzer)

        @todo:
        - add unit testing
    */
}
