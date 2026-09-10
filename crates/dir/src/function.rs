use crate::{
    block::{BlockId, BlockTable},
    types::TypeId,
};
use drop_util::table::*;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub block_id: BlockId,
    pub return_type: TypeId,
}

impl Function {
    pub fn get_all_blocks(&self, table: &BlockTable) -> Vec<BlockId> {
        let mut blocks = Vec::<BlockId>::new();

        let mut block_id = Some(self.block_id);
        while let Some(id) = block_id {
            blocks.push(id);
            block_id = table[id].next;
        }

        blocks
    }
}

pub type FunctionTable = Table<Function>;
pub type FunctionId = TableId<Function>;
