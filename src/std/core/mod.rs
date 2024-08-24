use std::{cell::RefCell, rc::Rc};

use io::StdCoreIo;

use crate::interpreter::env::Env;

pub mod io;

pub fn load_core(env: Rc<RefCell<Env>>) {
    let mut io = StdCoreIo::new(env);
    io.load();
}
