use drop_core::codegen::*;
use drop_core::lexer::*;
use drop_core::parser::*;

fn main() {
    let code = r#"
main :: fn () i32 {
    return 123;
}
        "#;
    let tokens = Lexer::new(code).tokenize();

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(node) => {
            let mut codegen = Codegen::new(node);
            match codegen.generate() {
                Ok(_) => println!("ok"),
                Err(e) => println!("Codegen error: {}", e),
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
