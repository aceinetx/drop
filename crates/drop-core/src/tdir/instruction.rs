use drop_util::table::TableId;

#[derive(Debug)]
pub struct Instruction {
    pub ty: Option<Ref>,
    pub kind: InstructionKind,
}

impl From<InstructionKind> for Instruction {
    fn from(value: InstructionKind) -> Self {
        Self {
            ty: None,
            kind: value,
        }
    }
}

#[derive(Debug)]
pub enum InstructionKind {
    Func(Func),
    Block(Vec<Ref>),
    Number(i64),
    Return(Ref),
}

pub type Index = TableId<Instruction>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ref {
    TypeU8(),
    TypeI32(),
    TypeConst(),
    TypePtr(),
    TypeFuncRet(),
    Index(Index),
}

impl From<Index> for Ref {
    fn from(value: Index) -> Self {
        Ref::Index(value)
    }
}

#[derive(Debug)]
pub struct Func {
    pub is_extern: bool,
    pub name: String,
    pub args: Vec<(String, Ref)>,
    pub return_type: Ref,
    pub body: Option<Ref>,
}
