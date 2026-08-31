use drop_core::codegen::*;
use drop_core::lexer::*;
use drop_core::parser::*;

fn main() -> Result<(), String> {
    let code = r#"
fn add (x: i32, y: i32) i32 {
    return 2 + 2 * 2;
}

fn main () i32 {
    return 123;
}
        "#;
    let tokens = Lexer::new(code).tokenize();

    let mut parser = Parser::new(tokens);
    let node = parser.parse()?;
    println!("{:#?}", node);
    let mut codegen = Codegen::new(node);
    codegen.generate()?;

    Ok(())
}
