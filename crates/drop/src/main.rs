use std::collections::VecDeque;
use std::{env, error::Error, path::PathBuf};

use drop_core::{lexer::tokenize, parser::parse, tdir, udir};

#[derive(Default)]
struct Options {
    print_tokens: bool,
    print_ast: bool,
    print_udir: bool,
    print_tdir: bool,
    filename: String,
}

fn run_compiler(options: &Options) -> Result<(), Box<dyn Error>> {
    let filename = std::fs::canonicalize(PathBuf::from(&options.filename))?;

    let code = std::fs::read_to_string(filename)?;

    let tokens = tokenize(&code);
    if options.print_tokens {
        print!("\x1b[33m");
        println!("{:#?}", tokens);
    }
    let node = parse(&tokens)?;
    if options.print_ast {
        print!("\x1b[92m");
        println!("{:#?}", node);
    }
    let udir = udir::generate(&node).unwrap();
    if options.print_udir {
        print!("\x1b[35m");
        println!("{:#?}", udir);
    }
    let tdir = tdir::generate(&udir);
    if options.print_tdir {
        print!("\x1b[96m");
        println!("{:#?}", tdir);
    }
    print!("\x1b[0m");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: VecDeque<String> = env::args().collect();
    args.remove(0);

    let mut options = Options::default();

    while let Some(arg) = args.pop_front() {
        match arg.as_str() {
            "-print-tokens" => {
                options.print_tokens = true;
            }
            "-print-ast" => {
                options.print_ast = true;
            }
            "-print-udir" => {
                options.print_udir = true;
            }
            "-print-tdir" => {
                options.print_tdir = true;
            }
            other => {
                if other.starts_with("/-") {
                    continue;
                } else if other.starts_with('-') {
                    return Err(format!("invalid argument {other}").into());
                }
                options.filename = other.to_string();
            }
        }
    }

    run_compiler(&options)
}
