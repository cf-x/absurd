use super::*;
use crate::ast::*;


#[test]
fn stmt_1() {
    let left = vec![Statement::Break {}];
    let right = get_ast("break;");

    assert_eq!(left, right, "testing `break;`");
}
