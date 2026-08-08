use crate::{block::*, function::*, instruction::*, types::*, value::Value};

pub type InsertPoint = InstructionId;

#[derive(Debug)]
pub struct IR {
    pub(crate) function_table: FunctionTable,
    pub(crate) block_table: BlockTable,
    pub(crate) instruction_table: InstructionTable,
    pub(crate) insert_point: InsertPoint,
    pub(crate) type_u0: TypeId,
    pub(crate) type_u8: TypeId,
    pub(crate) type_u16: TypeId,
    pub(crate) type_u32: TypeId,
    pub(crate) type_u64: TypeId,
    pub(crate) type_i8: TypeId,
    pub(crate) type_i16: TypeId,
    pub(crate) type_i32: TypeId,
    pub(crate) type_i64: TypeId,
    pub(crate) type_table: TypeTable,
    next_value_id: usize,
}

impl Default for IR {
    fn default() -> Self {
        let mut type_table = TypeTable::new();
        Self {
            function_table: FunctionTable::new(),
            block_table: BlockTable::new(),
            instruction_table: InstructionTable::new(),
            insert_point: InsertPoint::new(0),
            type_u0: type_table.insert(Type::U0),
            type_u8: type_table.insert(Type::U8),
            type_u16: type_table.insert(Type::U16),
            type_u32: type_table.insert(Type::U32),
            type_u64: type_table.insert(Type::U64),
            type_i8: type_table.insert(Type::I8),
            type_i16: type_table.insert(Type::I16),
            type_i32: type_table.insert(Type::I32),
            type_i64: type_table.insert(Type::I64),
            type_table,
            next_value_id: 0,
        }
    }
}

impl IR {
    pub(crate) fn get_next_value_id(&mut self) -> usize {
        let id = self.next_value_id;
        self.next_value_id += 1;
        id
    }

    pub fn create_function(&mut self, name: &str, return_type: TypeId) -> FunctionId {
        let nop = self.instruction_table.insert(Instruction {
            kind: InstructionKind::Nop,
            next: None,
            block: BlockId::new(0), // initialized later
            out: None,
        });

        let block = self.block_table.insert(Block {
            begin_instruction: nop,
            next: None,
            function: FunctionId::new(0), // initialized later
        });

        self.instruction_table[nop].block = block;

        let function = self.function_table.insert(Function {
            name: name.to_string(),
            block_id: block,
            return_type,
        });

        self.block_table[block].function = function;

        self.set_insert_function(function);

        function
    }

    pub fn set_insert_instruction(&mut self, id: InstructionId) {
        _ = &self.instruction_table[id];
        self.insert_point = id;
    }

    pub fn set_insert_block(&mut self, id: BlockId) {
        let block = &self.block_table[id];
        let mut inst_id = block.begin_instruction;
        while self.instruction_table[inst_id].next.is_some() {
            inst_id = self.instruction_table[inst_id].next.unwrap();
        }
        self.set_insert_instruction(inst_id);
    }

    pub fn set_insert_function(&mut self, id: FunctionId) {
        let function = &self.function_table[id];
        let mut block_id = function.block_id;
        while self.block_table[block_id].next.is_some() {
            block_id = self.block_table[block_id].next.unwrap();
        }
        self.set_insert_block(block_id);
    }

    pub fn insert(&mut self, mut instruction: Instruction) -> InstructionId {
        let current_id = self.insert_point;
        instruction.block = self.instruction_table[current_id].block;
        let new_id = self.instruction_table.insert(instruction);
        self.insert_point = new_id;
        self.instruction_table[new_id].next = self.instruction_table[current_id].next;
        self.instruction_table[current_id].next = Some(new_id);

        new_id
    }

    pub fn insert_kind(&mut self, out: Option<Value>, kind: InstructionKind) -> InstructionId {
        self.insert(Instruction {
            kind,
            next: None,
            block: BlockId::new(0),
            out,
        })
    }
}
