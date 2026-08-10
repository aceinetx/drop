use crate::token::*;

pub struct Lexer<'a> {
    code: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self { code, pos: 0 }
    }

    pub fn tokenize(&mut self) -> TokenStream {
        let mut stream = TokenStream::default();
        self.pos = 0;
        loop {
            let token = self.next();
            let eof = matches!(token, Token::Eof);
            stream.tokens.push(token);
            if eof {
                break;
            }
        }
        stream
    }

    fn next(&mut self) -> Token {
        let mut c: char;
        while {
            c = self.ch();
            c != 0 as char
        } {
            if c.is_alphabetic() {
                let begin = self.pos - 1;
                let mut end = self.pos;

                while {
                    c = self.ch();
                    c.is_alphanumeric()
                } {
                    end = self.pos;
                }
                self.pos -= 1;

                let ident = String::from(&self.code[begin..end]);

                macro_rules! keyword {
                    ( $str: expr, $val:expr ) => {
                        if ident == $str {
                            return $val;
                        }
                    };
                }

                keyword!("extern", Token::Extern);
                keyword!("fn", Token::Fn);
                keyword!("return", Token::Return);
                keyword!("const", Token::Const);

                return Token::Identifier(ident);
            } else if c == '"' {
                let begin = self.pos;
                let mut end = self.pos;

                while {
                    c = self.ch();
                    c != '"'
                } {
                    end = self.pos;
                }

                let string = String::from(&self.code[begin..end]);

                return Token::String(string);
            } else if c.is_numeric() {
                let mut num = 0i64;
                while c.is_numeric() {
                    num *= 10;
                    num += (c as u8 - '0' as u8) as i64;
                    c = self.ch();
                }
                self.pos -= 1;

                return Token::Number(num);
            } else if c == ':' {
                return Token::Colon;
            } else if c == ';' {
                return Token::Semicolon;
            } else if c == '(' {
                return Token::Lparen;
            } else if c == ')' {
                return Token::Rparen;
            } else if c == '*' {
                return Token::Star;
            } else if c == '{' {
                return Token::Lbrace;
            } else if c == '}' {
                return Token::Rbrace;
            } else if c == ',' {
                return Token::Comma;
            } else if c == '[' {
                return Token::Lbracket;
            } else if c == ']' {
                return Token::Rbracket;
            }
            assert!(c.is_whitespace(), "Invalid character: {}", c);
        }
        Token::Eof
    }

    fn ch(&mut self) -> char {
        let c = self.code.chars().nth(self.pos).unwrap_or(0 as char);
        self.pos += 1;
        c
    }
}
