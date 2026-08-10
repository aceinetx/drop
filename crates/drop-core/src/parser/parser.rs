use crate::{
    parser::{Node, NodeKind, ParserResult},
    token::{Token, TokenIterator, TokenStream},
};

pub struct Parser {
    tokens: TokenStream,
}

macro_rules! expect_token {
    ($self:expr, $tok:pat) => {{
        let tok = $self.tokens.next();
        if !matches!(tok, $tok) {
            Err(format!(
                "expected {}, but found {:?}",
                stringify!($tok),
                tok
            ))
        } else {
            Ok(())
        }
    }};
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens }
    }

    fn parse_type(&mut self) -> ParserResult<Node> {
        match self.tokens.next() {
            Token::Identifier(name) => Ok(Node::new(NodeKind::TypeRef(name.clone()))),
            Token::Star => {
                let underlying = Box::new(self.parse_type()?);
                Ok(Node::new(NodeKind::TypePtr(underlying)))
            }
            Token::Const => {
                let underlying = Box::new(self.parse_type()?);
                Ok(Node::new(NodeKind::TypeConst(underlying)))
            }
            Token::Lbracket => {
                expect_token!(self, Token::Rbracket)?;
                let underlying = Box::new(self.parse_type()?);
                Ok(Node::new(NodeKind::TypeSlice(underlying)))
            }
            token => Err(format!(
                "expected identifier, '*' or const in a type, but found {:?}",
                token
            )),
        }
    }

    fn parse_statement(&mut self) -> ParserResult<Node> {
        match self.tokens.next() {
            Token::Return => {
                let node = self.parse_expr()?;
                expect_token!(self, Token::Semicolon)?;
                Ok(Node::new(NodeKind::Return(Box::new(node))))
            }
            token => panic!("{:?}", token),
        }
    }

    fn parse_block(&mut self) -> ParserResult<Node> {
        expect_token!(self, Token::Lbrace)?;
        let mut nodes = Vec::<Node>::new();
        while !matches!(self.tokens.peek(), Token::Rbrace) {
            nodes.push(self.parse_statement()?);
        }
        expect_token!(self, Token::Rbrace)?;
        Ok(Node::new(NodeKind::Block(nodes)))
    }

    fn parse_fn_arguments(&mut self) -> ParserResult<Vec<(String, Node)>> {
        match self.tokens.next() {
            Token::Rparen => Ok(vec![]),
            Token::Identifier(name) => {
                let name = name.clone();

                expect_token!(self, Token::Colon)?;
                let arg_type = self.parse_type()?;
                let mut vec = vec![(name, arg_type)];

                match self.tokens.next() {
                    Token::Rparen => Ok(vec),
                    Token::Comma => {
                        vec.append(&mut self.parse_fn_arguments()?);
                        Ok(vec)
                    }
                    token => Err(format!(
                        "expected rparen or comma in function arguments, but found {:?}",
                        token
                    )),
                }
            }
            token => Err(format!(
                "expected rparen or identifier in function arguments, but found {:?}",
                token
            )),
        }
    }

    fn parse_fn(&mut self) -> ParserResult<Node> {
        let is_extern: bool;
        let extern_name: String;
        match self.tokens.next() {
            Token::Extern => {
                is_extern = true;
                expect_token!(self, Token::Fn)?;
                extern_name = match self.tokens.next() {
                    Token::Identifier(name) => name.clone(),
                    token => {
                        return Err(format!("expected extern name, but found {:?}", token));
                    }
                };
            }
            Token::Fn => {
                is_extern = false;
                extern_name = String::new();
            }
            token => {
                return Err(format!(
                    "expected extern or fn in function definition, but found {:?}",
                    token
                ));
            }
        }

        expect_token!(self, Token::Lparen)?;
        let args = self.parse_fn_arguments()?;

        let return_type = Box::new(self.parse_type()?);

        let body = if is_extern {
            expect_token!(self, Token::Semicolon)?;
            None
        } else {
            Some(Box::new(self.parse_block()?))
        };

        let kind = NodeKind::FunctionDef {
            is_extern,
            extern_name,
            args,
            return_type,
            body,
        };

        Ok(Node::new(kind))
    }

    fn parse_number(&mut self) -> ParserResult<Node> {
        match self.tokens.next() {
            Token::Number(num) => Ok(Node::new(NodeKind::Number(*num))),
            token => Err(format!("expected a number, but found {:?}", token)),
        }
    }

    fn parse_expr(&mut self) -> ParserResult<Node> {
        match self.tokens.peek() {
            Token::Extern => self.parse_fn(),
            Token::Fn => self.parse_fn(),
            Token::Number(_) => self.parse_number(),
            _ => todo!(),
        }
    }

    fn parse_tld(&mut self) -> ParserResult<Node> {
        let name = match self.tokens.next() {
            Token::Identifier(name) => name.clone(),
            token => return Err(format!("expected identifier in tld, but found {:?}", token)),
        };
        expect_token!(self, Token::Colon)?;
        expect_token!(self, Token::Colon)?;
        let value = Box::new(self.parse_expr()?);

        let node = Node::new(NodeKind::CompAssign { name, value });
        Ok(node)
    }

    pub fn parse(&mut self) -> ParserResult<Node> {
        let mut nodes = Vec::<Node>::new();
        loop {
            match self.tokens.peek() {
                Token::Eof => break,
                _ => nodes.push(self.parse_tld()?),
            }
        }
        Ok(Node::new(NodeKind::Root(nodes)))
    }
}
