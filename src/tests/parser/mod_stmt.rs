use super::*;
use crate::ast::*;

#[test]
fn stmt_1() {
    let left = vec![Statement::Mod {
        src: "\"source\"".to_string(),
    }];
    let right = get_ast("mod \"source\";");

    assert_eq!(left, right, "testing `mod \"source\";`");
}
