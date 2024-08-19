use bundler::interpreter_raw;
mod analyzer;
mod ast;
mod bundler;
mod env;
mod errors;
mod expr;
mod interpreter;
mod lexer;
mod literals;
mod parser;
mod resolver;
mod std;
#[cfg(test)]
mod tests;

fn main() {
    let src = r#"
    let a: number = 5;
    let x: any = print(++a);
        "#;
    interpreter_raw(src);
}
