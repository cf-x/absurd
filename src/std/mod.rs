use std::{cell::RefCell, rc::Rc};
pub mod core;

use crate::{
    ast::{DeclrFuncType, FuncValType, LiteralType},
    env::Env,
};

pub fn func(name: &str, arity: usize, env: &mut Rc<RefCell<Env>>, func: Rc<dyn FuncValType>) {
    env.borrow().define(
        name.to_string(),
        LiteralType::DeclrFunc(DeclrFuncType {
            name: name.to_string(),
            arity,
            func,
        }),
    )
}
