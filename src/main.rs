use drop::{Lexer, Token};

fn main() {
    let code = r#"
puts :: extern fn puts (fmt: *const u8) i32;

main :: fn () i32 {
    return puts("Hello, World!");
}
        "#;
    let mut lexer = Lexer::new(&code);

    loop {
        let token = lexer.next();
        println!("{:?}", token);
        if matches!(token, Token::Eof()) {
            return;
        }
    }
}
