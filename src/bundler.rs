use crate::{
    ast::Statement,
    errors::Error,
    interpreter::{env::Env, expr::Expression, Interpreter},
    parser::{scanner::Scanner, Parser},
    resolver::Resolver,
    Config,
};
use std::{cell::RefCell, rc::Rc};

/// Function `parser` parses the source into the AST representad as `Vec<Statement>`:
pub fn parser(src: &str, err: Error) -> Vec<Statement> {
    let mut lexer = Scanner::new(src, err.clone());
    let tokens = lexer.scan();
    let mut parser = Parser::new(tokens.clone(), err);
    parser.parse()
}

/// treats source as an single expression, and parses source into the expression
pub fn parse_expr(src: &str, err: Error) -> Expression {
    let mut lexer = Scanner::new(src, err.clone());
    let tokens = lexer.scan();
    let mut parser = Parser::new(tokens.clone(), err);
    parser.expr()
}

/// interpets the source code based on the input configuration
pub fn interpreter(src: &str, project: Config) {
    let err = Error::new(src);
    let stmts = parser(src, err.clone());
    let mut int = Interpreter::new(project.clone(), err.clone());
    let mut resolver = Resolver::new(err.clone());
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect(), 0);
}

/// interpets the source code as an module
pub fn interpreter_mod(
    src: &str,
    mod_src: Option<String>,
    env: Rc<RefCell<Env>>,
) -> Rc<RefCell<Env>> {
    let err = Error::new(src);
    let mut int = Interpreter::new_with_env(env, true, src, mod_src, 0);
    let stmts = parser(src, err.clone());
    let mut resolver = Resolver::new(err);
    let locals = resolver.resolve(&stmts, &mut int.env);
    int.env.borrow_mut().resolve(locals);
    int.interpret(stmts.iter().collect(), 0)
}
