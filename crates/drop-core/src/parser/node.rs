#[derive(Debug)]
pub enum BinopKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct NodeFuncDef {
    pub is_extern: bool,
    pub name: String,
    pub args: Vec<(String, Node)>,
    pub return_type: Box<Node>,
    pub body: Option<Box<Node>>,
}

#[derive(Debug)]
pub enum NodeKind {
    None,
    Root(Vec<Node>),
    FunctionDef(NodeFuncDef),
    TypeRef(String),
    TypePtr(Box<Node>),
    TypeConst(Box<Node>),
    TypeSlice(Box<Node>),
    Block(Vec<Node>),
    Number(i64),
    Return(Option<Box<Node>>),
    VarRef(String),
    String(String),
    Binop(Box<Node>, BinopKind, Box<Node>),
    Call(Box<Node>, Vec<Node>),
    Import(String),
}

#[derive(Debug)]
pub struct Node {
    pub node: NodeKind,
}

impl Node {
    pub fn new(node: NodeKind) -> Self {
        Self { node }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(NodeKind::None)
    }
}
