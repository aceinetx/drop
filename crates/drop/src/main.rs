use dir::*;

fn main() {
    let mut ir = IR::default();
    _ = ir.create_function("main", ir.get_type_i32());
    let value = ir.constant_signed(ir.get_type_i32(), 0);
    let value = ir.constant_signed(ir.get_type_i32(), 1);
    ir.ret(Some(value));
    dbg!(&ir);
    match ir.emit() {
        Ok(code) => println!("code:\n{}", code),
        Err(e) => println!("{:?}", e),
    }
}

/*
use drop_core::codegen::*;
use drop_core::lexer::*;
use drop_core::parser::*;

fn main() {
    let code = r#"
puts :: extern fn puts (fmt: *const u8) i32;
sum :: fn (ints: [] i32) i32 {
}

main :: fn () i32 {
    return puts("Hello, World!");
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
*/
