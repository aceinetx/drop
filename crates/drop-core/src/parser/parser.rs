use crate::{
    parser::{BinopKind, Node, NodeKind, ParserResult},
    token::Token,
};

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    token_index: usize,
}

macro_rules! expect_token {
    ($self:expr, $tok:pat) => {{
        let tok = $self.next_token();
        if !matches!(tok, $tok) {
            dbg!("err here");
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

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            token_index: 0,
        }
    }

    fn next_token(&mut self) -> &Token {
        let token = self.tokens.get(self.token_index).unwrap_or(&Token::Eof);
        self.token_index += 1;
        token
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.get(self.token_index).unwrap_or(&Token::Eof)
    }

    fn parse_type(&mut self) -> ParserResult<Node> {
        match self.next_token() {
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
        match self.next_token() {
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
        while !matches!(self.peek_token(), Token::Rbrace) {
            nodes.push(self.parse_statement()?);
        }
        expect_token!(self, Token::Rbrace)?;
        Ok(Node::new(NodeKind::Block(nodes)))
    }

    fn parse_fn_arguments(&mut self) -> ParserResult<Vec<(String, Node)>> {
        match self.next_token() {
            Token::Rparen => Ok(vec![]),
            Token::Identifier(name) => {
                let name = name.clone();

                expect_token!(self, Token::Colon)?;
                let arg_type = self.parse_type()?;
                let mut vec = vec![(name, arg_type)];

                match self.next_token() {
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
        let is_extern = match self.next_token() {
            Token::Extern => {
                expect_token!(self, Token::Fn)?;
                true
            }
            Token::Fn => false,
            token => {
                return Err(format!(
                    "expected extern or fn in function definition, but found {:?}",
                    token
                ));
            }
        };
        let name = match self.next_token() {
            Token::Identifier(name) => name.clone(),
            token => {
                return Err(format!("expected extern name, but found {:?}", token));
            }
        };

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
            name,
            args,
            return_type,
            body,
        };

        Ok(Node::new(kind))
    }

    fn parse_primary(&mut self) -> ParserResult<Node> {
        match self.next_token() {
            Token::Number(num) => Ok(Node::new(NodeKind::Number(*num))),
            Token::Identifier(ident) => Ok(Node::new(NodeKind::VarRef(ident.clone()))),
            Token::String(str) => Ok(Node::new(NodeKind::String(str.clone()))),
            token => Err(format!(
                "expected a number or identifier, but found {:?}",
                token
            )),
        }
    }

    fn parse_fn_call_arguments(&mut self) -> ParserResult<Vec<Node>> {
        match self.peek_token() {
            Token::Rparen => {
                _ = self.next_token();
                Ok(vec![])
            }
            _ => {
                let mut vec = vec![self.parse_expr()?];
                match self.next_token() {
                    Token::Comma => {
                        vec.append(&mut self.parse_fn_call_arguments()?);
                        Ok(vec)
                    }
                    Token::Rparen => Ok(vec),
                    token => Err(format!(
                        "expected semicolon or rparen, but found {:?}",
                        token
                    )),
                }
            }
        }
    }

    fn parse_postfix(&mut self) -> ParserResult<Node> {
        let mut node = self.parse_primary()?;

        #[allow(clippy::single_match)]
        match self.peek_token() {
            Token::Lparen => {
                _ = self.next_token();
                node = Node::new(NodeKind::Call(
                    Box::new(node),
                    self.parse_fn_call_arguments()?,
                ))
            }
            _ => (),
        }

        Ok(node)
    }

    fn parse_multiplicative(&mut self) -> ParserResult<Node> {
        let mut lhs = self.parse_postfix()?;

        loop {
            lhs = match self.peek_token() {
                Token::Star => {
                    self.next_token();

                    let rhs = Box::new(self.parse_postfix()?);

                    Node::new(NodeKind::Binop(Box::new(lhs), BinopKind::Mul, rhs))
                }
                Token::Div => {
                    self.next_token();

                    let rhs = Box::new(self.parse_postfix()?);

                    Node::new(NodeKind::Binop(Box::new(lhs), BinopKind::Div, rhs))
                }
                _ => return Ok(lhs),
            };
        }
    }

    fn parse_additive(&mut self) -> ParserResult<Node> {
        let mut lhs = self.parse_multiplicative()?;

        loop {
            lhs = match self.peek_token() {
                Token::Plus => {
                    self.next_token();

                    let rhs = Box::new(self.parse_multiplicative()?);

                    Node::new(NodeKind::Binop(Box::new(lhs), BinopKind::Add, rhs))
                }
                Token::Minus => {
                    self.next_token();

                    let rhs = Box::new(self.parse_multiplicative()?);

                    Node::new(NodeKind::Binop(Box::new(lhs), BinopKind::Sub, rhs))
                }
                _ => return Ok(lhs),
            };
        }
    }

    fn parse_expr(&mut self) -> ParserResult<Node> {
        self.parse_additive()
    }

    fn parse_tld(&mut self) -> ParserResult<Node> {
        self.parse_fn()
    }

    pub fn parse(&mut self) -> ParserResult<Node> {
        self.token_index = 0;
        let mut nodes = Vec::<Node>::new();
        loop {
            match self.peek_token() {
                Token::Eof => break,
                _ => nodes.push(self.parse_tld()?),
            }
        }
        Ok(Node::new(NodeKind::Root(nodes)))
    }
}

pub fn parse(tokens: &Vec<Token>) -> ParserResult<Node> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
