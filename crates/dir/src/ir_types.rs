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

    fn insert_reused_type(&mut self, insert_type: Type) -> TypeId {
        for id in self.type_table.all() {
            let ty = &self.type_table[id];
            if *ty == insert_type {
                return id;
            }
        }

        self.type_table.insert(insert_type)
    }

    fn verify_tuple_has_no_void(&self, tuple: &Type) -> bool {
        if let Type::Tuple(tuple) = tuple {
            for id in tuple.iter() {
                let ty = &self.type_table[*id];
                if matches!(ty, Type::U0) {
                    return false;
                }
            }
        }
        true
    }

    pub fn create_tuple(&mut self, types: Vec<TypeId>, reuse: bool) -> TypeId {
        let tuple = Type::Tuple(types);
        assert!(
            self.verify_tuple_has_no_void(&tuple),
            "A tuple must not contain a U0 field"
        );
        if reuse {
            self.insert_reused_type(tuple)
        } else {
            self.type_table.insert(tuple)
        }
    }

    pub fn create_ptr(&mut self, to: TypeId) -> TypeId {
        self.insert_reused_type(Type::Pointer(to))
    }

    pub fn create_const(&mut self, to: TypeId) -> TypeId {
        self.insert_reused_type(Type::Const(to))
    }
}
