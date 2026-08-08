use crate::types::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub(crate) id: usize,
    pub(crate) ty: TypeId,
}

impl Value {
    pub fn new(id: usize, ty: TypeId) -> Self {
        Self { id, ty }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_type(&self) -> TypeId {
        self.ty
    }
}
