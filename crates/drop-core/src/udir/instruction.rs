use drop_util::table::TableId;

#[derive(Debug)]
pub enum Instruction {
    Func(Func),
    Block(Vec<Ref>),
    Ret(Ref),
    Number(i64),
    Coerce(Coerce),
    Add(Ref, Ref),
    Sub(Ref, Ref),
    Mul(Ref, Ref),
}

pub type Index = TableId<Instruction>;

#[derive(Debug)]
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
pub struct Coerce {
    pub val: Ref,
    pub ty: Ref,
}

#[derive(Debug)]
pub struct Func {
    pub is_extern: bool,
    pub name: String,
    pub args: Vec<(String, Ref)>,
    pub return_type: Ref,
    pub body: Option<Ref>,
}
