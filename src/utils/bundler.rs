use super::errors::Error;
use super::manifest::Project;
use crate::ast::Statement;
use crate::interpreter::env::Env;
use crate::interpreter::Interpreter;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use std::cell::RefCell;
use std::rc::Rc;

use std::time::Instant;

pub fn parser(src: &str, err: Error, log: bool) -> Vec<Statement> {
    let mut start = None;
    if log {
        start = Some(Instant::now());
    }
    let mut lexer = Scanner::new(src, err.clone());
    let tokens = lexer.scan();
    if log {
        let scan_duration = start.unwrap().elapsed();
        println!("Scanner took: {:?}", scan_duration);
    }
    let mut start = None;
    if log {
        start = Some(Instant::now());
    }
    let mut parser = crate::parser::Parser::new(tokens.clone(), err);
    let stmts = parser.parse();
    if log {
        let parse_duration = start.unwrap().elapsed();
        println!("Parser took: {:?}", parse_duration);
    }
    stmts
}

pub fn interpreter_raw(src: &str, project: Project, log: bool) {
    let err = Error::new(src, project.clone());

    let stmts = parser(src, err.clone(), log);

    let mut start = None;
    if log {
        start = Some(Instant::now());
    }
    let mut int = Interpreter::new(project.clone(), err.clone());
    let mut resolver = Resolver::new(err.clone());
    let locals = resolver.resolve(&stmts, &mut int.env);
    if log {
        let resolve_duration = start.unwrap().elapsed();
        println!("Resolver took: {:?}", resolve_duration);
    }

    let mut start = None;
    if log {
        start = Some(Instant::now());
    }

    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect());
    if log {
        let interpret_duration = start.unwrap().elapsed();
        println!("Interpreter took: {:?}", interpret_duration);
    }
}

pub fn interpreter_mod(
    src: &str,
    mod_src: Option<String>,
    env: Rc<RefCell<Env>>,
    project: Project,
) -> Rc<RefCell<Env>> {
    let err = Error::new(src, project.clone());
    let mut int = Interpreter::new_with_env(env, true, src, mod_src);
    let stmts = parser(src, err.clone(), false);
    let mut resolver = Resolver::new(err);
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect())
}
