use drop_core::{ir1::ir1_gen, ir2::ir2_gen, lexer::tokenize, parser::parse};

#[test]
fn test_compilation_1() {
    let code = r#"
fn sixty_nine() i32 {
  return 69;
}

fn main () i32 {
  return puts("Hello, World!");
}

fn add (a: i32, b: i32) i32 {
  return a + b;
}

extern fn puts (s: *const u8) i32;
"#;

    let tokens = tokenize(&code);
    let root = parse(&tokens).expect("Parsing failed");
    eprintln!("{:#?}", root);

    let ir1 = ir1_gen(root);
    let ir2 = ir2_gen(ir1);
    eprintln!("{:#?}", ir2);
}
