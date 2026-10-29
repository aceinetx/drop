use drop_util::table::*;

#[derive(Debug)]
pub enum Type {
    IntegerLike(u32 /* bits */),
    TypePtr(TypeId),
    TypeConst(TypeId),
}

pub type TypeTable = Table<Type>;
pub type TypeId = TableId<Type>;
