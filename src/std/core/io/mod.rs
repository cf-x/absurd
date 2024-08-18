mod print;
use crate::env::Env;

pub struct StdCoreIo {
    env: Env,
}

impl StdCoreIo {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
    pub fn load(&mut self) {
        self.load_print();
    }
}
