/*
    print(text) - prints text to the terminal
*/

use std::rc::Rc;

use crate::{
    ast::{LiteralType, Wrapper},
    std::func,
};

use super::StdCoreIo;

impl StdCoreIo {
    pub fn load_print(&mut self) {
        func(
            "print",
            1,
            &mut self.env,
            Rc::new(Wrapper {
                0: Box::new(|args: &[LiteralType]| {
                    println!("{}", args[0].to_string());
                    LiteralType::Void
                }),
            }),
        );
    }
}
