use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{LiteralType, Token, Wrapper},
    errors::raw,
    interpreter::env::Env,
    std::func,
};
use colored::Colorize;

pub struct StdCoreTest {
    env: Rc<RefCell<Env>>,
    is_test: bool,
}

impl StdCoreTest {
    pub fn new(env: Rc<RefCell<Env>>, is_test: bool) -> Self {
        Self { env, is_test }
    }

    pub fn load(&mut self) {
        self.load_assert(None);
    }

    pub fn load_assert(&mut self, name: Option<Token>) {
        let name = match name {
            Some(n) => n.lexeme.clone(),
            None => "assert".to_string(),
        };

        if self.is_test {
            let mut env = Rc::clone(&self.env);
            let test_instance = Rc::new(RefCell::new(self.clone()));

            func(
                name.as_str(),
                1,
                &mut env,
                Rc::new(Wrapper {
                    0: Box::new(move |args: &[Option<LiteralType>]| {
                        let test_instance = Rc::clone(&test_instance);
                        let test_instance = test_instance.borrow_mut();
                        if args.len() != 2 {
                            raw("expected 2 arguments");
                        }
                        if args[0].clone().unwrap_or(LiteralType::Boolean(false))
                            != LiteralType::Boolean(true)
                        {
                            test_instance.fail(
                                args[1]
                                    .clone()
                                    .unwrap_or(LiteralType::String("unknown".to_string()))
                                    .to_string(),
                            );
                        } else {
                            test_instance.success(
                                args[1]
                                    .clone()
                                    .unwrap_or(LiteralType::String("unknown".to_string()))
                                    .to_string(),
                            );
                        }
                        LiteralType::Void
                    }),
                }),
            );
        }
    }

    // #[allow(dead_code)]
    #[inline]
    fn success(&self, name: String) {
        println!("  {}", format!("success: test '{}'", name).green());
    }

    // #[allow(dead_code)]
    #[inline]
    fn fail(&self, name: String) {
        println!("  {}", format!("fail: test '{}'", name).red());
    }
}

impl Clone for StdCoreTest {
    fn clone(&self) -> Self {
        Self {
            env: Rc::clone(&self.env),
            is_test: self.is_test,
        }
    }
}
