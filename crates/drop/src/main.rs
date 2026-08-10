use drop_core::codegen::*;
use drop_core::lexer::*;
use drop_core::parser::*;

fn main() -> Result<(), String> {
    let code = r#"
main :: fn () i32 {
    return 123;
}
        "#;
    let tokens = Lexer::new(code).tokenize();

    let mut parser = Parser::new(tokens);
    let node = parser.parse()?;
    let mut codegen = Codegen::new(node);
    codegen.generate()?;

    Ok(())
}
