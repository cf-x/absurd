use super::manifest::Project;
use crate::analyzer::Analyzer;
use crate::ast::{Statement, Token};
use crate::interpreter::env::Env;
use crate::interpreter::Interpreter;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use crate::utils::errors::Error;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn lexer(src: &str, project: Project) -> Vec<Token> {
    let err = Error::new(src, project.clone());
    let mut lexer = Scanner::new(src.to_string(), err.clone());
    lexer.scan()
}

pub fn parser(src: &str, project: Project) -> Vec<Statement> {
    let err = Error::new(src, project.clone());
    let mut lexer = Scanner::new(src.to_string(), err.clone());
    let tokens = lexer.scan();
    let mut parser = crate::parser::Parser::new(tokens, err.clone());
    parser.parse()
}

#[allow(dead_code)]
pub fn analyzer(src: &str, project: Project) -> Vec<Statement> {
    let err = Error::new(src, project.clone());
    let mut lexer = Scanner::new(src.to_string(), err.clone());
    let tokens = lexer.scan();
    let mut parser = crate::parser::Parser::new(tokens, err.clone());
    let stmts = parser.parse();
    let mut analyzer = Analyzer::new(stmts);
    analyzer.analyze()
}

pub fn interpreter_raw(src: &str, project: Project) {
    let err = Error::new(src, project.clone());
    let mut int = Interpreter::new(project.clone(), err.clone());
    let stmts = parser(src, project.clone());
    let mut resolver = Resolver::new(err.clone());
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect());
}

pub fn interpreter_mod(
    src: &str,
    mod_src: Option<String>,
    env: Rc<RefCell<Env>>,
    project: Project,
) -> Rc<RefCell<Env>> {
     let err = Error::new(src, project.clone());
    let mut int = Interpreter::new_with_env(env, true, src, mod_src);
    let stmts = parser(src, project.clone());
    let mut resolver = Resolver::new(err);
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect())
}
