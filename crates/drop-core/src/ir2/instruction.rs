use drop_util::table::*;

#[derive(Debug)]
pub struct FuncDefArg {
    pub name: String,
    pub ty: InstructionId,
}

#[derive(Debug)]
pub struct FuncDef {
    pub is_extern: bool,
    pub name: String,
    pub args: Vec<FuncDefArg>,
    pub body: Option<InstructionId>,
    pub return_type: InstructionId,
}

#[derive(Debug)]
pub enum Instruction {
    TypeRef(String),
    TypeConst(InstructionId),
    TypePtr(InstructionId),
    FuncDef(FuncDef),
    Number(i64),
    String(String),
    Call(InstructionId, Vec<InstructionId>),
    Block(Vec<InstructionId>),
    Return(Option<InstructionId>),
}

pub type InstructionTable = Table<Instruction>;
pub type InstructionId = TableId<Instruction>;
