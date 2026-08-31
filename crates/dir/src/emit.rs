use crate::{
    IR, block::BlockId, function::FunctionId, instruction::*, types::TypeId, value::Value,
};
use std::io;

impl IR {
    fn emit_instruction(&mut self, id: InstructionId) -> Result<String, String> {
        let inst = &self.instruction_table[id];
        match inst.kind {
            InstructionKind::Nop => Ok("while(0);".to_string()),
            InstructionKind::Ret(value) => {
                if let Some(value) = value {
                    Ok(format!("return _{};", value.id))
                } else {
                    Ok("return;".to_string())
                }
            }
            InstructionKind::ConstantSigned(value) => {
                Ok(format!("_{}={value};", inst.out.unwrap().id))
            }

            InstructionKind::ConstantUnsigned(value) => {
                Ok(format!("_{}={value};", inst.out.unwrap().id))
            }
            InstructionKind::Add(left, right) => Ok(format!(
                "_{}=_{}+_{};",
                inst.out.unwrap().id,
                left.id,
                right.id
            )),
            InstructionKind::Sub(left, right) => Ok(format!(
                "_{}=_{}-_{};",
                inst.out.unwrap().id,
                left.id,
                right.id
            )),
            InstructionKind::Mul(left, right) => Ok(format!(
                "_{}=_{}*_{};",
                inst.out.unwrap().id,
                left.id,
                right.id
            )),
            InstructionKind::Div(left, right) => Ok(format!(
                "_{}=_{}/_{};",
                inst.out.unwrap().id,
                left.id,
                right.id
            )),
        }
    }

    fn emit_block(&mut self, id: BlockId) -> Result<String, String> {
        let mut code = format!("_{}:\n", id.get());

        for inst in self.block_table[id].get_all_instructions(&self.instruction_table) {
            code += &self.emit_instruction(inst)?;
            code += "\n";
        }

        Ok(code)
    }

    fn emit_declaration(&self, value: &Value) -> Result<String, String> {
        Ok(format!("{} _{};", value.ty.get_c_id(), value.id))
    }

    fn emit_function(&mut self, id: FunctionId) -> Result<String, String> {
        let function = &self.function_table[id];

        let mut code = format!(
            "{} {}(){{\n",
            function.return_type.get_c_id(),
            function.name
        );

        for block in function.get_all_blocks(&self.block_table) {
            for inst in self.block_table[block].get_all_instructions(&self.instruction_table) {
                if let Some(value) = self.instruction_table[inst].out {
                    code.push_str(&self.emit_declaration(&value)?);
                    code.push('\n');
                }
            }
        }

        for block in function.get_all_blocks(&self.block_table) {
            code.push_str(&self.emit_block(block)?);
        }

        code.push_str("\n}\n");

        Ok(code)
    }

    fn emit_type(&mut self, id: TypeId) -> Result<String, String> {
        let ty = &self.type_table[id];
        let name = id.get_c_id();
        match ty {
            crate::types::Type::U0 => Ok(format!("typedef void {};", name)),
            crate::types::Type::U8 => Ok(format!("typedef uint8_t {};", name)),
            crate::types::Type::U16 => Ok(format!("typedef uint16_t {};", name)),
            crate::types::Type::U32 => Ok(format!("typedef uint32_t {};", name)),
            crate::types::Type::U64 => Ok(format!("typedef uint64_t {};", name)),
            crate::types::Type::I8 => Ok(format!("typedef int8_t {};", name)),
            crate::types::Type::I16 => Ok(format!("typedef int16_t {};", name)),
            crate::types::Type::I32 => Ok(format!("typedef int32_t {};", name)),
            crate::types::Type::I64 => Ok(format!("typedef int64_t {};", name)),
            crate::types::Type::Tuple(types) => {
                let mut sb = String::from("typedef struct{");
                for (i, id) in types.iter().enumerate() {
                    sb += &format!("{} _{};", id.get_c_id(), i);
                }
                sb += &format!("}}{};", name);
                Ok(sb)
            }
            _ => todo!(),
        }
    }

    pub fn emit(&mut self) -> Result<String, String> {
        if let Err(e) = std::fs::create_dir(".dropbuild")
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e.to_string());
        }

        let mut code = String::from("#include<stdint.h>\n");
        for ty in self.type_table.all() {
            code.push_str(&self.emit_type(ty)?);
            code.push('\n');
        }

        for func in self.function_table.all() {
            code.push_str(&self.emit_function(func)?);
        }

        Ok(code)
    }
}
