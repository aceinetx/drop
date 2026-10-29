use crate::{
    ir1,
    ir2::{instruction::*, types::*},
};

#[derive(Debug)]
pub struct IR2 {
    ir1: ir1::IR1,
    instructions: InstructionTable,
    types: TypeTable,
}

impl IR2 {
    pub fn new(ir1: ir1::IR1) -> Self {
        Self {
            ir1,
            instructions: InstructionTable::new(),
            types: TypeTable::new(),
        }
    }

    pub fn generate(&mut self) {}
}

pub fn ir2_gen(ir1: ir1::IR1) -> IR2 {
    let mut ir2 = IR2::new(ir1);
    ir2.generate();

    ir2
}
