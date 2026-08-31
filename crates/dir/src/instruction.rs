use crate::{block::BlockId, table::*, value::Value};

#[derive(Debug)]
pub enum InstructionKind {
    Nop,
    Ret(Option<Value>),
    ConstantSigned(i64),
    ConstantUnsigned(u64),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub out: Option<Value>,
    pub next: Option<InstructionId>,
    pub block: BlockId,
}

pub type InstructionTable = Table<Instruction>;
pub type InstructionId = TableId<Instruction>;
