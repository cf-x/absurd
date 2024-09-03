use core::{io::StdCoreIo, load_core, test::StdCoreTest};
use std::{cell::RefCell, rc::Rc};
pub mod core;
pub mod literal;

use literal::{load_literal, number::StdLiteralNumber, string::StdLiteralString};

use crate::{
    ast::{DeclrFuncType, FuncValType, LiteralType, Token},
    errors::raw,
    interpreter::{
        env::{Env, FuncKind},
        Interpreter,
    },
};

pub fn func(name: &str, arity: usize, env: &mut Rc<RefCell<Env>>, func: Rc<dyn FuncValType>) {
    let params = vec![];
    env.borrow().define_pub_func(
        name.to_string(),
        LiteralType::DeclrFunc(DeclrFuncType {
            name: name.to_string(),
            arity,
            func,
        }),
        FuncKind {
            params,
            is_async: false,
            is_pub: false,
        },
    )
}

impl Interpreter {
    pub fn load_std(&self, src: String, names: Vec<(Token, Option<Token>)>, all: bool) {
        let parts: Vec<&str> = src.split("::").collect();

        if parts[0] != "std" {
            raw(format!("standard library '{src}' doesn't exist").as_str());
            return;
        }

        if parts.len() < 2 {
            raw("standard library module not specified");
            return;
        }

        match parts[1] {
            "core" => self.load_core_module(&parts, names, all),
            "literal" => self.load_literal_module(&parts, names, all),
            _ => raw(format!("std module '{}' doesn't exist", parts[1]).as_str()),
        }
    }

    fn load_core_module(&self, parts: &[&str], names: Vec<(Token, Option<Token>)>, all: bool) {
        if parts.len() < 3 {
            load_core(self.env.clone(), self.project.test);
            return;
        }

        match parts[2] {
            "io" => self.load_core_io(names, all),
            "test" => self.load_core_test(names, all),
            _ => raw(format!("std module 'core::{}' doesn't exist", parts[2]).as_str()),
        }
    }

    fn load_core_test(&self, names: Vec<(Token, Option<Token>)>, all: bool) {
        let mut std_core_test = StdCoreTest::new(self.env.clone(), self.project.test);
        if all {
            std_core_test.load();
        } else {
            for (name, name2) in names {
                match name.lexeme.as_str() {
                    "assert" => std_core_test.load_assert(name2),
                    // "assert_eq" => std_core_test.load_assert_eq(name2),
                    // "assert_ne" => std_core_test.load_assert_ne(name2),
                    // "assert_gt" => std_core_test.load_assert_gt(name2),
                    // "assert_lt" => std_core_test.load_assert_lt(name2),
                    // "assert_ge" => std_core_test.load_assert_ge(name2),
                    // "assert_le" => std_core_test.load_assert_le(name2),
                    _ => raw(
                        format!("std module 'core::test::{}' doesn't exist", name.lexeme).as_str(),
                    ),
                }
            }
        }
    }

    fn load_core_io(&self, names: Vec<(Token, Option<Token>)>, all: bool) {
        let mut std_core_io = StdCoreIo::new(self.env.clone());
        if all {
            std_core_io.load();
        } else {
            for (name, name2) in names {
                match name.lexeme.as_str() {
                    "print" => std_core_io.load_print(name2),
                    "eprint" => std_core_io.load_eprint(name2),
                    "warn" => std_core_io.load_warn(name2),
                    "panic" => std_core_io.load_panic(name2),
                    "exit" => std_core_io.load_exit(name2),
                    "read_num" => std_core_io.load_read_num(name2),
                    "read_str" => std_core_io.load_read_str(name2),
                    "read_char" => std_core_io.load_read_char(name2),
                    "read_bool" => std_core_io.load_read_bool(name2),
                    _ => raw(
                        format!("std module 'core::io::{}' doesn't exist", name.lexeme).as_str(),
                    ),
                }
            }
        }
    }

    fn load_literal_module(&self, parts: &[&str], names: Vec<(Token, Option<Token>)>, all: bool) {
        if parts.len() < 3 {
            load_literal(self.env.clone());
            return;
        }

        match parts[2] {
            "number" => self.load_literal_number(names, all),
            "string" => self.load_literal_string(names, all),
            _ => raw(format!("std module 'literal::{}' doesn't exist", parts[2]).as_str()),
        }
    }

    fn load_literal_string(&self, names: Vec<(Token, Option<Token>)>, all: bool) {
        let mut std = StdLiteralString::new(self.env.clone());
        if all {
            std.load();
        }

        for (name, name2) in names {
            match name.lexeme.as_str() {
                "chars_count" => std.load_chars_count(name2),
                "contains" => std.load_contains(name2),
                "find" => std.load_find(name2),
                "ends_with" => std.load_ends_with(name2),
                "starts_with" => std.load_starts_with(name2),
                "is_empty" => std.load_is_empty(name2),
                "len" => std.load_len(name2),
                "to_uppercase" => std.load_to_uppercase(name2),
                "to_lowercase" => std.load_to_lowercase(name2),
                "trim" => std.load_trim(name2),
                "trim_end" => std.load_trim_end(name2),
                "trim_start" => std.load_trim_start(name2),
                "replace" => std.load_replace(name2),
                _ => raw(format!(
                    "std module 'literal::string::{}' doesn't exist",
                    name.lexeme
                )
                .as_str()),
            }
        }
    }

    fn load_literal_number(&self, names: Vec<(Token, Option<Token>)>, all: bool) {
        let mut std = StdLiteralNumber::new(self.env.clone());
        if all {
            std.load();
        }

        for (name, name2) in names {
            match name.lexeme.as_str() {
                "sqr" => std.load_sqr(name2),
                "add" => std.load_add(name2),
                "sub" => std.load_sub(name2),
                "mult" => std.load_mult(name2),
                "div" => std.load_div(name2),
                "rem" => std.load_rem(name2),
                "sqrt" => std.load_sqrt(name2),
                "cbrt" => std.load_cbrt(name2),
                "pow" => std.load_pow(name2),
                "log" => std.load_log(name2),
                "sin" => std.load_sin(name2),
                "asin" => std.load_asin(name2),
                "cos" => std.load_cos(name2),
                "acos" => std.load_acos(name2),
                "tan" => std.load_tan(name2),
                "atan" => std.load_atan(name2),
                "abs" => std.load_abs(name2),
                "floor" => std.load_floor(name2),
                "ceil" => std.load_ceil(name2),
                "round" => std.load_round(name2),
                "signum" => std.load_signum(name2),
                "hypot" => std.load_hypot(name2),
                "exp" => std.load_exp(name2),
                "exp2" => std.load_exp2(name2),
                "exp_m1" => std.load_exp_m1(name2),
                "ln" => std.load_ln(name2),
                "max" => std.load_max(name2),
                "min" => std.load_min(name2),
                "avg" => std.load_avg(name2),
                "to_degrees" => std.load_to_degrees(name2),
                "to_radians" => std.load_to_radians(name2),
                _ => raw(format!(
                    "std module 'literal::number::{}' doesn't exist",
                    name.lexeme
                )
                .as_str()),
            }
        }
    }
}
