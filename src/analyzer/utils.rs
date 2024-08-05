use super::*;
use crate::ast::{Statement, Token};

impl Analyzer {
    pub fn next(&mut self) -> Statement {
        let mut ast = self.input_ast.clone();
        ast.reverse();
        if !self.is_eof() {
            return ast.get(self.crnt + 1).unwrap().clone();
        }
        ast.get(self.crnt).unwrap().clone()
    }

    pub fn is_eof(&mut self) -> bool {
        if self.input_ast.clone().len() <= self.crnt {
            return true;
        }
        false
    }

    pub fn is_called(&self, kind: Called, name: Token) -> bool {
        match kind {
            Called::Var(_) => self
                .called
                .iter()
                .any(|c| matches!(c, Called::Var(n) if *n == name)),
            Called::Func(_) => self
                .called
                .iter()
                .any(|c| matches!(c, Called::Func(n) if n.name == name)),
        }
    }

    pub fn push_called(&mut self, callee: Called) {
        self.called.push(callee);
    }
}
