use bundler::interpreter_raw;

mod analyzer;
mod ast;
mod bundler;
mod env;
mod errors;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod resolver;
#[cfg(test)]
mod tests;

fn main() {
    let src = r#"
    func name() -> string {
        return "hi";
    }
    print(name());
        "#;
    interpreter_raw(src);
}
