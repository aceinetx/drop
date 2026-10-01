use crate::{
    parser::{BinopKind, Node},
    udir::instruction::{Coerce, Func, Index, Instruction, Ref},
};
use drop_util::table::Table;

pub type UDIR = Table<Instruction>;

#[derive(Debug)]
pub enum GenerateError {
    UnknownType(String),
}

type GenerateResult<T> = Result<T, GenerateError>;

pub struct UDIRGen<'a> {
    instructions: UDIR,
    ast: &'a Node,
}

impl<'a> UDIRGen<'a> {
    pub fn new(ast: &'a Node) -> Self {
        Self {
            instructions: Table::new(),
            ast,
        }
    }

    fn gen_node(&mut self, node: &Node) -> GenerateResult<Ref> {
        match node {
            Node::FunctionDef {
                is_extern,
                name,
                args,
                return_type,
                body,
            } => {
                let mut gen_args = Vec::<(String, Ref)>::new();
                for arg in args.iter() {
                    gen_args.push((arg.0.clone(), self.gen_node(&arg.1)?));
                }
                let return_type = self.gen_node(return_type)?;
                let body = if !*is_extern {
                    Some(self.gen_node(body.as_ref().unwrap())?)
                } else {
                    None
                };

                let instruction = Instruction::Func(Func {
                    is_extern: *is_extern,
                    name: name.to_string(),
                    args: gen_args,
                    return_type,
                    body,
                });

                Ok(Ref::Index(self.instructions.insert(instruction)))
            }
            Node::Block(vec) => {
                let mut refs = Vec::<Ref>::new();
                for node in vec.iter() {
                    refs.push(self.gen_node(node)?);
                }

                let instruction = Instruction::Block(refs);
                Ok(Ref::Index(self.instructions.insert(instruction)))
            }
            Node::Return(node) => {
                let expr_ref = self.gen_node(node)?;

                let inst: Ref = self
                    .instructions
                    .insert(Instruction::Coerce(Coerce {
                        val: expr_ref,
                        ty: Ref::TypeFuncRet(),
                    }))
                    .into();

                let inst: Ref = self.instructions.insert(Instruction::Ret(inst)).into();

                Ok(inst)
            }
            Node::Number(number) => {
                let instruction = Instruction::Number(*number);
                Ok(Ref::Index(self.instructions.insert(instruction)))
            }
            Node::Binop(lhs, op, rhs) => {
                let lhs = self.gen_node(lhs)?;
                let rhs = self.gen_node(rhs)?;

                match op {
                    BinopKind::Add => {
                        Ok(self.instructions.insert(Instruction::Add(lhs, rhs)).into())
                    }
                    BinopKind::Sub => {
                        Ok(self.instructions.insert(Instruction::Sub(lhs, rhs)).into())
                    }
                    BinopKind::Mul => {
                        Ok(self.instructions.insert(Instruction::Mul(lhs, rhs)).into())
                    }
                    BinopKind::Div => {
                        panic!("div")
                    }
                }
            }
            Node::TypeRef(name) => match name.as_str() {
                "i32" => Ok(Ref::TypeI32()),
                "u8" => Ok(Ref::TypeU8()),
                _ => Err(GenerateError::UnknownType(name.to_string())),
            },
            _ => unimplemented!("{:?}", node),
        }
    }

    pub fn generate(&mut self) -> GenerateResult<()> {
        let Node::Root(root) = &self.ast else {
            panic!()
        };

        for node in root.iter() {
            self.gen_node(node)?;
        }

        Ok(())
    }
}

pub fn generate(ast: &Node) -> GenerateResult<UDIR> {
    let mut udir = UDIRGen::new(ast);
    udir.generate()?;
    Ok(udir.instructions)
}
