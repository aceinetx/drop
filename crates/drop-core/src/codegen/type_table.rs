use std::collections::HashMap;

use crate::codegen::{Type, TypeId, TypeKind};

#[derive(Debug, Default)]
pub struct TypeTable {
    pub types: HashMap<TypeId, Type>,
    next_id: TypeId,
}

impl TypeTable {
    fn get_id(&mut self) -> TypeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_builtins(&mut self) {
        let mut types = vec![
            Type {
                kind: TypeKind::U8(),
                c_id: "u8".to_string(),
                is_builtin: true,
            },
            Type {
                kind: TypeKind::I32(),
                c_id: "i32".to_string(),
                is_builtin: true,
            },
            Type {
                kind: TypeKind::USZ(),
                c_id: "usz".to_string(),
                is_builtin: true,
            },
        ];

        for it in types.drain(..) {
            self.insert(it);
        }
    }

    pub fn get_builtin(&mut self, name: &str) -> Option<TypeId> {
        for (id, it) in self.types.iter() {
            if it.is_builtin && it.c_id == name {
                return Some(*id);
            }
        }
        None
    }

    pub fn insert(&mut self, r#type: Type) -> TypeId {
        for (id, it) in self.types.iter() {
            if it.c_id == r#type.c_id {
                return *id;
            }
        }

        let id = self.get_id();
        self.types.insert(id, r#type);
        id
    }

    pub fn get(&self, id: TypeId) -> Option<&Type> {
        self.types.get(&id)
    }

    pub fn get_mut(&mut self, id: TypeId) -> Option<&mut Type> {
        self.types.get_mut(&id)
    }
}
