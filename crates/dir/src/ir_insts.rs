use crate::{IR, instruction::*, types::*, value::Value};

impl IR {
    pub fn ret(&mut self, value: Option<Value>) {
        self.insert_kind(None, InstructionKind::Ret(value));
    }

    pub fn constant_signed(&mut self, type_id: TypeId, value: i64) -> Value {
        let ty = &self.type_table[type_id];
        assert!(
            ty.is_signed_int(),
            "constant_signed function requires type to be a signed integer"
        );

        let out = Value::new(self.get_next_value_id(), type_id);
        self.insert_kind(Some(out), InstructionKind::ConstantSigned(value));
        out
    }

    pub fn constant_unsigned(&mut self, type_id: TypeId, value: u64) -> Value {
        let ty = &self.type_table[type_id];
        assert!(
            ty.is_unsigned_int(),
            "constant_unsigned function requires type to be an unsigned integer"
        );

        let out = Value::new(self.get_next_value_id(), type_id);
        self.insert_kind(Some(out), InstructionKind::ConstantUnsigned(value));
        out
    }
}
