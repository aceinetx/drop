use std::{env, error::Error, path::PathBuf};

use drop_core::{lexer::tokenize, parser::parse, udir};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filename = std::fs::canonicalize(PathBuf::from(filename))?;

    let code = std::fs::read_to_string(filename)?;

    let tokens = tokenize(&code);
    let node = parse(&tokens)?;
    println!("{:#?}", node);
    let udir = udir::generate(&node).unwrap();

    println!("{:#?}", udir);

    Ok(())
}
