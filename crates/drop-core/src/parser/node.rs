#[derive(Debug)]
pub enum BinopKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Node {
    None,
    Root(Vec<Node>),
    FunctionDef {
        is_extern: bool,
        name: String,
        args: Vec<(String, Node)>,
        return_type: Box<Node>,
        body: Option<Box<Node>>,
    },
    TypeRef(String),
    TypePtr(Box<Node>),
    TypeConst(Box<Node>),
    TypeSlice(Box<Node>),
    Block(Vec<Node>),
    Number(i64),
    Return(Box<Node>),
    VarRef(String),
    String(String),
    Binop(Box<Node>, BinopKind, Box<Node>),
    Call(Box<Node>, Vec<Node>),
    Import(String),
}
