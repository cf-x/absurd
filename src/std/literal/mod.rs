pub mod number;
pub mod string;
use crate::interpreter::env::Env;
use number::StdLiteralNumber;
use std::{cell::RefCell, rc::Rc};
use string::StdLiteralString;

pub fn load_literal(env: Rc<RefCell<Env>>) {
    let mut number = StdLiteralNumber::new(env.clone());
    number.load();
    let mut string = StdLiteralString::new(env);
    string.load();
}
