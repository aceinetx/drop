use crate::codegen::*;
use crate::parser::{Node, NodeKind};

impl Codegen {
    fn create_ptr_type(&mut self, underlying: TypeId) -> TypeId {
        let underlying_cid = &self.type_table.get(underlying).unwrap().c_id;
        self.type_table.insert(Type::new(
            TypeKind::Pointer(underlying),
            format!("p{}", underlying_cid),
        ))
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
            NodeKind::CompAssign { name: _, value } => {
                self.resolve_types(value)?;
            }
            NodeKind::FunctionDef {
                is_extern: _,
                extern_name: _,
                args,
                return_type,
                body: _,
            } => {
                // Resolve return type
                self.resolve_types(return_type)?;
                let return_type_id = return_type.resolved_type.unwrap();

                let mut cid = format!("fn_{}", return_type_id);

                // Resolve argument types
                let mut arg_types = Vec::<TypeId>::new();
                for (_, node) in args.iter_mut() {
                    self.resolve_types(node)?;
                    let id = node.resolved_type.unwrap();
                    arg_types.push(id);

                    // Add the id to cid
                    cid.push('_');
                    cid.push_str(&id.to_string());
                }

                node.resolved_type = Some(self.type_table.insert(Type::new(
                    TypeKind::Function {
                        args: arg_types,
                        return_type: return_type_id,
                    },
                    cid,
                )));
            }
            NodeKind::TypeRef(name) => {
                // Match by builtin types first
                if let Some(id) = self.type_table.get_builtin(name) {
                    node.resolved_type = Some(id);
                }
            }
            NodeKind::TypePtr(underlying) => {
                self.resolve_types(underlying)?;
                let underlying_id = underlying.resolved_type.unwrap();
                node.resolved_type = Some(self.create_ptr_type(underlying_id));
            }
            NodeKind::TypeConst(underlying) => {
                self.resolve_types(underlying)?;
                let underlying_id = underlying.resolved_type.unwrap();
                let underlying_cid = &self.type_table.get(underlying_id).unwrap().c_id;
                node.resolved_type = Some(self.type_table.insert(Type::new(
                    TypeKind::Const(underlying_id),
                    format!("c{}", underlying_cid),
                )));
            }
            NodeKind::TypeSlice(underlying) => {
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
            }
            _ => unreachable!(),
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
