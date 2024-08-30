use std::{cell::RefCell, rc::Rc};

use io::StdCoreIo;
use test::StdCoreTest;

use crate::interpreter::env::Env;

pub mod io;
pub mod test;

pub fn load_core(env: Rc<RefCell<Env>>, is_test: bool) {
    let mut io = StdCoreIo::new(env.clone());
    io.load();
    let mut test = StdCoreTest::new(env.clone(), is_test);
    test.load()
}
