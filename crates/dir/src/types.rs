use drop_util::table::*;

#[derive(Debug, PartialEq)]
pub enum Type {
    U0,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Pointer(TypeId),
    Const(TypeId),
    Tuple(Vec<TypeId>),
}

impl Type {
    pub fn is_unsigned_int(&self) -> bool {
        matches!(self, Type::U8 | Type::U16 | Type::U32 | Type::U64)
    }

    pub fn is_ptr(&self) -> bool {
        matches!(self, Type::Pointer(_))
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64)
    }
}

pub type TypeTable = Table<Type>;
pub type TypeId = TableId<Type>;

pub fn type_id_get_c_id(type_id: TypeId) -> String {
    format!("dir_ty_{}", type_id.get())
}
