use super::errors::Error;
use super::manifest::Project;
use crate::ast::Statement;
use crate::interpreter::env::Env;
use crate::interpreter::Interpreter;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use std::cell::RefCell;
use std::rc::Rc;

pub fn parser(src: &str, err: Error) -> Vec<Statement> {
    let mut lexer = Scanner::new(src.to_string(), err.clone());
    let tokens = lexer.scan();
    let mut parser = crate::parser::Parser::new(tokens, err);
    parser.parse()
}

pub fn interpreter_raw(src: &str, project: Project) {
    let err = Error::new(src, project.clone());
    let mut int = Interpreter::new(project.clone(), err.clone());
    let stmts = parser(src, err.clone());
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
    let stmts = parser(src, err.clone());
    let mut resolver = Resolver::new(err);
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect())
}
