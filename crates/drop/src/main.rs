use std::env;
use std::path::PathBuf;

use drop_core::codegen::*;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filename = std::fs::canonicalize(PathBuf::from(filename)).map_err(|e| e.to_string())?;
    let mut codegen = Codegen::create_from_path(filename)?;

    codegen.generate(true)?;

    Ok(())
}
