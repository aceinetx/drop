use crate::table::*;

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
        match self {
            Type::U8 => true,
            Type::U16 => true,
            Type::U32 => true,
            Type::U64 => true,
            _ => false,
        }
    }

    pub fn is_signed_int(&self) -> bool {
        match self {
            Type::I8 => true,
            Type::I16 => true,
            Type::I32 => true,
            Type::I64 => true,
            _ => false,
        }
    }
}

pub type TypeTable = Table<Type>;
pub type TypeId = TableId<Type>;

impl TypeId {
    pub fn get_c_id(&self) -> String {
        format!("dir_ty_{}", self.get())
    }
}
