use drop_core::{ir1::ir1_gen, lexer::tokenize, parser::parse};

#[test]
fn test_compilation_1() {
    let code = r#"
fn sixty_nine() i32 {
  return 69;
}

fn main () i32 {
  return puts("Hello, World!");
}

extern fn puts (s: *const u8) i32;
"#;

    let tokens = tokenize(&code);
    let root = parse(&tokens).expect("Parsing failed");
    eprintln!("{:#?}", root);

    let ir1 = ir1_gen(root);
}
