use crate::{
    IR,
    types::{Type, TypeId},
};

impl IR {
    pub fn get_type_u0(&self) -> TypeId {
        self.type_u0
    }

    pub fn get_type_u8(&self) -> TypeId {
        self.type_u8
    }

    pub fn get_type_u16(&self) -> TypeId {
        self.type_u16
    }

    pub fn get_type_u32(&self) -> TypeId {
        self.type_u32
    }

    pub fn get_type_u64(&self) -> TypeId {
        self.type_u64
    }

    pub fn get_type_i8(&self) -> TypeId {
        self.type_i8
    }

    pub fn get_type_i16(&self) -> TypeId {
        self.type_i16
    }

    pub fn get_type_i32(&self) -> TypeId {
        self.type_i32
    }

    pub fn get_type_i64(&self) -> TypeId {
        self.type_i64
    }

    pub fn create_tuple(&mut self, types: Vec<TypeId>) -> TypeId {
        self.type_table.insert(Type::Tuple(types))
    }
}
