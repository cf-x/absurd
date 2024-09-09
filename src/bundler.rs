// bundles multiple parts together
use coloredpp::Colorize;
use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{
    ast::Statement,
    errors::Error,
    interpreter::{env::Env, expr::Expression, Interpreter},
    manifest::Project,
    parser::{scanner::Scanner, Parser},
    resolver::Resolver,
};

pub fn parser(src: &str, err: Error, log: bool) -> Vec<Statement> {
    let mut start = None;
    if log {
        println!("{}", "scanning...".yellow());
        start = Some(Instant::now());
    }
    let mut lexer = Scanner::new(src, err.clone(), log);
    let tokens = lexer.scan();
    if log {
        let scan_duration = start.unwrap().elapsed();
        let text = format!("{:?}", scan_duration);
        println!("{} {}", "completed scanning in".green(), text.blue());
    }
    let mut start = None;
    if log {
        println!("{}", "parsing...".yellow());
        start = Some(Instant::now());
    }
    let mut parser = Parser::new(tokens.clone(), err, log);
    let stmts = parser.parse();
    if log {
        let parse_duration = start.unwrap().elapsed();
        let text = format!("{:?}", parse_duration);
        println!("{} {}", "completed parsing in".green(), text.blue());
    }
    stmts
}

pub fn parse_expr(src: &str, err: Error) -> Expression {
    let mut lexer = Scanner::new(src, err.clone(), false);
    let tokens = lexer.scan();
    let mut parser = Parser::new(tokens.clone(), err, false);
    parser.expr()
}

pub fn interpreter_raw(src: &str, project: Project, log: bool) {
    let full_start = Instant::now();
    let err = Error::new(src, project.clone());

    let stmts = parser(src, err.clone(), log);

    let mut start = None;
    if log {
        println!("{}", "resolving...".yellow());
        start = Some(Instant::now());
    }
    let mut int = Interpreter::new(project.clone(), err.clone());
    let mut resolver = Resolver::new(err.clone());
    let locals = resolver.resolve(&stmts, &mut int.env);
    if log {
        let resolver_duration = start.unwrap().elapsed();
        let text = format!("{:?}", resolver_duration);
        println!("{} {}", "completed resolving in".green(), text.blue());
    }

    let mut start = None;
    if log {
        println!("{}", "interpreting...".yellow());
        start = Some(Instant::now());
    }

    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect());
    if log {
        let interpreter_duration = start.unwrap().elapsed();
        let text = format!("{:?}", interpreter_duration);
        println!("{} {}", "completed interpreting in".green(), text.blue());
        let total_duration = full_start.elapsed();
        let text = format!("{:?}", total_duration);
        println!("{} {}", "total time elapsed:".green(), text.blue());
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
