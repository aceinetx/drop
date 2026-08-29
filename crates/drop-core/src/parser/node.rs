use dir::types::TypeId;

#[derive(Debug)]
pub enum NodeKind {
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
}

#[derive(Debug)]
pub struct Node {
    pub node: NodeKind,
    pub resolved_type: Option<TypeId>,
}

impl Node {
    pub fn new(node: NodeKind) -> Self {
        Self {
            node,
            resolved_type: None,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(NodeKind::None)
    }
}
