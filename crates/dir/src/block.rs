use crate::{IR, function::FunctionId, instruction::InstructionId, table::*};

#[derive(Debug)]
pub struct Block {
    pub begin_instruction: InstructionId,
    pub next: Option<BlockId>,
    pub function: FunctionId,
}

impl Block {
    pub fn get_all_instructions(&self, ir: &IR) -> Vec<InstructionId> {
        let mut insts = Vec::<InstructionId>::new();

        let mut inst_id = Some(self.begin_instruction);
        while let Some(id) = inst_id {
            insts.push(id);
            inst_id = ir.instruction_table[id].next;
        }

        insts
    }
}

pub type BlockTable = Table<Block>;
pub type BlockId = TableId<Block>;
