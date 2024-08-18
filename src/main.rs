use bundler::interpreter_raw;

mod analyzer;
mod ast;
mod bundler;
mod env;
mod errors;
mod expr;
mod interpreter;
mod lexer;
mod std;
mod parser;
mod resolver;
#[cfg(test)]
mod tests;

fn main() {
    let src = r#"
    let x: any = print("Hello, World!");
        "#;
    interpreter_raw(src);
}
