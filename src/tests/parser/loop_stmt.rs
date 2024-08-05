use super::*;

use crate::ast::*;
#[test]
fn stmt_2() {
    let left = vec![Statement::Loop {
        iter: Some(7),
        body: vec![Statement::Block { stmts: vec![] }],
    }];
    let right = get_ast("loop 0o7 {}");

    assert_eq!(left, right, "testing `loop 0o7 {{}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Loop {
        iter: None,
        body: vec![Statement::Block { stmts: vec![] }],
    }];
    let right = get_ast("loop {}");

    assert_eq!(left, right, "testing `loop {{}}`");
}