use std::io;

use crate::parser::{Node, NodeKind};
use dir::{IR, value::*};

pub struct Codegen {
    pub(crate) ast: Node,
    pub(crate) ir: IR,
}

impl Codegen {
    pub fn new(ast: Node) -> Self {
        Self {
            ast,
            ir: IR::default(),
        }
    }

    fn build(&mut self, node: &Node) -> Result<Option<Value>, String> {
        match &node.node {
            NodeKind::Root(vec) => {
                for node in vec.iter() {
                    self.build(node)?;
                }
                Ok(None)
            }
            NodeKind::FunctionDef {
                is_extern,
                name,
                args,
                return_type,
                body,
            } => {
                _ = self
                    .ir
                    .create_function(name, return_type.resolved_type.unwrap());
                let body = body.as_ref().unwrap();
                self.build(body)
            }
            NodeKind::Block(vec) => {
                for node in vec.iter() {
                    self.build(node)?;
                }
                Ok(None)
            }
            NodeKind::Return(value) => {
                let value = self.build(value)?;
                self.ir.ret(value);
                Ok(None)
            }
            NodeKind::Number(value) => Ok(Some(
                self.ir.constant_signed(self.ir.get_type_i64(), *value),
            )),
            kind => todo!("{:?}", kind),
        }
    }

    pub fn generate(&mut self) -> Result<(), String> {
        if let Err(e) = std::fs::create_dir(".dropbuild")
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e.to_string());
        }

        self.resolve_types_root()?;

        let ast = std::mem::take(&mut self.ast);
        self.build(&ast)?;
        self.ast = ast;

        let code = self.ir.emit()?;
        println!("{}", code);

        Ok(())
    }
}
