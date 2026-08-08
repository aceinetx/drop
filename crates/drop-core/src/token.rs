#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Identifier(String),
    String(String),
    Colon,
    Semicolon,
    Lparen,
    Rparen,
    Star,
    Lbrace,
    Rbrace,
    Extern,
    Return,
    Fn,
    Comma,
    Const,
    Lbracket,
    Rbracket,
}

#[derive(Default)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pos: usize,
}

pub trait TokenIterator {
    fn next(&mut self) -> &Token;
    fn peek(&mut self) -> &Token;
}

impl TokenIterator for TokenStream {
    fn next(&mut self) -> &Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::Eof);
        self.pos += 1;
        if self.pos >= self.tokens.len() {
            self.pos = self.tokens.len() - 1;
        }
        token
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
}
