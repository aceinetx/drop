use crate::codegen::*;
use crate::parser::{Node, NodeKind};
use dir::types::TypeId;

impl Codegen {
    fn create_ptr_type(&mut self, underlying: TypeId) -> TypeId {
        todo!();
        /*
        let underlying_cid = &self.type_table.get(underlying).unwrap().c_id;
        self.type_table.insert(Type::new(
            TypeKind::Pointer(underlying),
            format!("p{}", underlying_cid),
        ))
        */
    }

    pub(crate) fn resolve_types(&mut self, node: &mut Node) -> Result<(), String> {
        match &mut node.node {
            NodeKind::None => unreachable!(),
            NodeKind::Root(nodes) => {
                // Resolve children nodes
                for node in nodes.iter_mut() {
                    self.resolve_types(node)?;
                }
            }
            NodeKind::Number(_) => {
                node.resolved_type = Some(self.ir.get_type_i64());
            }
            NodeKind::Binop(left, _, right) => {
                self.resolve_types(left)?;
                self.resolve_types(right)?;
                if left.resolved_type.unwrap() != right.resolved_type.unwrap() {
                    return Err("types of binary operation don't match".into());
                }

                node.resolved_type = left.resolved_type;
            }
            NodeKind::Return(value) => {
                self.resolve_types(value)?;
            }
            NodeKind::Block(nodes) => {
                for node in nodes.iter_mut() {
                    self.resolve_types(node)?;
                }
            }
            NodeKind::FunctionDef {
                is_extern: _,
                name: _,
                args,
                return_type,
                body,
            } => {
                // Resolve return type
                self.resolve_types(return_type)?;
                if let Some(body) = body {
                    self.resolve_types(body)?;
                }
            }
            NodeKind::TypeRef(name) => {
                // Match by builtin types first
                if name == "u0" {
                    node.resolved_type = Some(self.ir.get_type_u0());
                } else if name == "u8" {
                    node.resolved_type = Some(self.ir.get_type_u8());
                } else if name == "u16" {
                    node.resolved_type = Some(self.ir.get_type_u16());
                } else if name == "u32" {
                    node.resolved_type = Some(self.ir.get_type_u32());
                } else if name == "u64" {
                    node.resolved_type = Some(self.ir.get_type_u64());
                } else if name == "i8" {
                    node.resolved_type = Some(self.ir.get_type_i8());
                } else if name == "i16" {
                    node.resolved_type = Some(self.ir.get_type_i16());
                } else if name == "i32" {
                    node.resolved_type = Some(self.ir.get_type_i32());
                } else if name == "i64" {
                    node.resolved_type = Some(self.ir.get_type_i64());
                }
            }
            NodeKind::TypePtr(underlying) => {
                todo!();
                /*
                self.resolve_types(underlying)?;
                let underlying_id = underlying.resolved_type.unwrap();
                node.resolved_type = Some(self.create_ptr_type(underlying_id));
                */
            }
            NodeKind::TypeConst(underlying) => {
                todo!();
                /*
                self.resolve_types(underlying)?;
                let underlying_id = underlying.resolved_type.unwrap();
                let underlying_cid = &self.type_table.get(underlying_id).unwrap().c_id;
                node.resolved_type = Some(self.type_table.insert(Type::new(
                    TypeKind::Const(underlying_id),
                    format!("c{}", underlying_cid),
                )));
                */
            }
            NodeKind::TypeSlice(underlying) => {
                todo!();
                /*
                self.resolve_types(underlying)?;
                let underlying_id = underlying.resolved_type.unwrap();
                let type_kind = TypeKind::Struct {
                    fields: vec![
                        ("ptr".to_string(), self.create_ptr_type(underlying_id)),
                        (
                            "len".to_string(),
                            self.type_table.get_builtin("usz").unwrap(),
                        ),
                    ],
                };
                let underlying_cid = &self.type_table.get(underlying_id).unwrap().c_id;
                node.resolved_type = Some(
                    self.type_table
                        .insert(Type::new(type_kind, format!("s{}", underlying_cid))),
                );
                */
            }
            NodeKind::Import(_) => (),
            other => unimplemented!("{:#?}", other),
        }
        Ok(())
    }

    pub(crate) fn resolve_types_root(&mut self) -> Result<(), String> {
        let mut ast = std::mem::take(&mut self.ast);
        self.resolve_types(&mut ast)?;
        self.ast = ast;

        Ok(())
    }
}
