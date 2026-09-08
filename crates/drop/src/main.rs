use std::env;
use std::fs;

use drop_core::codegen::*;
use drop_core::lexer::*;
use drop_core::parser::*;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let code = fs::read_to_string(filename).map_err(|e| e.to_string())?;

    let tokens = Lexer::new(&code).tokenize();

    let mut parser = Parser::new(tokens);
    let node = parser.parse()?;
    println!("{:#?}", node);
    let mut codegen = Codegen::new(node);
    codegen.generate()?;

    Ok(())
}
