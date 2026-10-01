use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filename = std::fs::canonicalize(PathBuf::from(filename))?;
    _ = filename;

    Ok(())
}
