use crate::ir1::instruction::*;
use crate::parser::{Node, NodeFuncDef, NodeKind};
use std::collections::HashMap;

pub struct IR1 {
    pub ast: Vec<Node>,
    pub instructions: InstructionTable,
    pub scopes: Vec<HashMap<String, InstructionId>>,
}

impl IR1 {
    pub fn new(ast: Vec<Node>) -> Self {
        Self {
            ast,
            instructions: InstructionTable::new(),
            scopes: vec![HashMap::new()],
        }
    }

    fn lookup_variable(&self, name: &str) -> Option<InstructionId> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return Some(scope[name]);
            }
        }
        None
    }

    fn generate_type(&mut self, ty: &NodeKind) -> Instruction {
        match ty {
            NodeKind::TypeRef(name) => Instruction::TypeRef(name.clone()),
            NodeKind::TypeConst(node) => {
                let inner = self.generate_type(&node.node);
                let id = self.instructions.insert(inner);
                Instruction::TypeConst(id)
            }
            NodeKind::TypePtr(node) => {
                let inner = self.generate_type(&node.node);
                let id = self.instructions.insert(inner);
                Instruction::TypePtr(id)
            }
            _ => panic!("generate_type accepted a non-type node"),
        }
    }

    fn generate_extern_funcdef(&mut self, funcdef: &NodeFuncDef) -> InstructionId {
        assert!(funcdef.is_extern);

        let mut args: Vec<FuncDefArg> = vec![];
        for arg in funcdef.args.iter() {
            let ty = self.generate_type(&arg.1.node);
            let ty_id = self.instructions.insert(ty);

            args.push(FuncDefArg {
                name: arg.0.clone(),
                ty: ty_id,
            });
        }

        let return_ty = self.generate_type(&funcdef.return_type.node);
        let return_ty_id = self.instructions.insert(return_ty);

        let inst = Instruction::FuncDef(FuncDef {
            is_extern: true,
            name: funcdef.name.clone(),
            args,
            body: None,
            return_type: return_ty_id,
        });

        let inst_id = self.instructions.insert(inst);

        self.scopes[0].insert(funcdef.name.clone(), inst_id);
        inst_id
    }

    fn generate_non_extern_funcdef(&mut self, funcdef: &NodeFuncDef) -> InstructionId {
        assert!(!funcdef.is_extern);

        let mut args: Vec<FuncDefArg> = vec![];
        for arg in funcdef.args.iter() {
            let ty = self.generate_type(&arg.1.node);
            let ty_id = self.instructions.insert(ty);

            args.push(FuncDefArg {
                name: arg.0.clone(),
                ty: ty_id,
            });
        }

        let return_ty = self.generate_type(&funcdef.return_type.node);
        let return_ty_id = self.instructions.insert(return_ty);

        let inst = Instruction::FuncDef(FuncDef {
            is_extern: true,
            name: funcdef.name.clone(),
            args,
            body: Some(
                self.generate_generic_node(
                    &funcdef
                        .body
                        .as_ref()
                        .expect("non extern function is expected to have a body")
                        .node,
                )
                .expect("generate_generic_node is expected to emit an instruction"),
            ),
            return_type: return_ty_id,
        });

        let inst_id = self.instructions.insert(inst);

        inst_id
    }

    fn generate_generic_node(&mut self, node: &NodeKind) -> Option<InstructionId> {
        match node {
            NodeKind::FunctionDef(funcdef) => {
                if !funcdef.is_extern
                /* we already generated declarations for these in Self::generate */
                {
                    Some(self.generate_non_extern_funcdef(&funcdef))
                } else {
                    None
                }
            }
            NodeKind::Block(vec) => {
                self.scopes.push(HashMap::new());

                let inst = Instruction::Block(
                    vec.iter()
                        .map(|x| self.generate_generic_node(&x.node).expect("generate_generic_node is expected to emit an instruction for every block node"))
                        .collect(),
                );

                self.scopes.pop();

                Some(self.instructions.insert(inst))
            }
            NodeKind::Return(node) => {
                let inst = Instruction::Return(match node {
                    Some(node) => self.generate_generic_node(&node.node),
                    None => None,
                });

                Some(self.instructions.insert(inst))
            }
            NodeKind::VarRef(name) => Some(
                self.lookup_variable(name)
                    .expect("TODO: real error messages for undeclared variables"),
            ),
            NodeKind::Call(function_value, node_args) => {
                let function_ref = self.generate_generic_node(&function_value.node).unwrap();

                let mut args: Vec<InstructionId> = vec![];
                for arg in node_args.iter() {
                    args.push(self.generate_generic_node(&arg.node).unwrap());
                }

                let inst = Instruction::Call(function_ref, args);

                Some(self.instructions.insert(inst))
            }
            NodeKind::String(s) => Some(self.instructions.insert(Instruction::String(s.clone()))),
            NodeKind::Number(n) => Some(self.instructions.insert(Instruction::Number(*n))),
            other => todo!("{:#?}", other),
        }
    }

    pub fn generate(&mut self) {
        let ast = std::mem::take(&mut self.ast);

        // generate declarations before definitions
        for node in ast.iter() {
            match &node.node {
                NodeKind::FunctionDef(funcdef) => {
                    if funcdef.is_extern {
                        self.generate_extern_funcdef(funcdef);
                    }
                }
                _ => (),
            }
        }

        eprintln!("==================== INSTRUCTIONS ==================== ");
        eprintln!("{:#?}", self.instructions);
        eprintln!("==================== SCOPES ==================== ");
        eprintln!("{:#?}", self.scopes);

        for node in ast.iter() {
            self.generate_generic_node(&node.node);
        }

        eprintln!("==================== INSTRUCTIONS ==================== ");
        eprintln!("{:#?}", self.instructions);

        self.ast = ast;
    }
}

pub fn ir1_gen(root: Node) -> IR1 {
    let NodeKind::Root(ast) = root.node else {
        panic!()
    };

    let mut ir1 = IR1::new(ast);
    ir1.generate();

    ir1
}
