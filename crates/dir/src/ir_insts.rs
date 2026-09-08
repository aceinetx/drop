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
            ty.is_unsigned_int() || ty.is_ptr(),
            "constant_unsigned function requires type to be an unsigned integer or a pointer"
        );

        let out = Value::new(self.get_next_value_id(), type_id);
        self.insert_kind(Some(out), InstructionKind::ConstantUnsigned(value));
        out
    }

    pub fn add(&mut self, left: Value, right: Value) -> Value {
        assert!(left.ty == right.ty);

        let out = Value::new(self.get_next_value_id(), left.ty);
        self.insert_kind(Some(out), InstructionKind::Add(left, right));
        out
    }

    pub fn sub(&mut self, left: Value, right: Value) -> Value {
        assert!(left.ty == right.ty);

        let out = Value::new(self.get_next_value_id(), left.ty);
        self.insert_kind(Some(out), InstructionKind::Sub(left, right));
        out
    }

    pub fn mul(&mut self, left: Value, right: Value) -> Value {
        assert!(left.ty == right.ty);

        let out = Value::new(self.get_next_value_id(), left.ty);
        self.insert_kind(Some(out), InstructionKind::Mul(left, right));
        out
    }

    pub fn div(&mut self, left: Value, right: Value) -> Value {
        assert!(left.ty == right.ty);

        let out = Value::new(self.get_next_value_id(), left.ty);
        self.insert_kind(Some(out), InstructionKind::Div(left, right));
        out
    }
}
