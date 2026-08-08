pub type TypeId = usize;

#[derive(Debug)]
pub enum TypeKind {
    I32(),
    U8(),
    USZ(),
    Struct {
        fields: Vec<(String, TypeId)>,
    },
    Function {
        args: Vec<TypeId>,
        return_type: TypeId,
    },
    Pointer(TypeId),
    Const(TypeId),
}

#[derive(Debug)]
pub struct Type {
    pub kind: TypeKind,
    pub c_id: String,
    pub is_builtin: bool,
}

impl Type {
    pub fn new(kind: TypeKind, c_id: String) -> Self {
        Self {
            kind,
            c_id,
            is_builtin: false,
        }
    }
}
