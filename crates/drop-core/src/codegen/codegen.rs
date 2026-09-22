use std::{collections::HashSet, io, path::PathBuf};

use crate::{
    lexer::tokenize,
    parser::{BinopKind, Node, NodeKind, parse},
};
use dir::{IR, value::*};

pub struct Codegen {
    pub(crate) ast: Node,
    pub(crate) ir: IR,
    pub(crate) file_path: PathBuf,
}

impl Codegen {
    pub fn new(file_path: PathBuf, ast: Node) -> Self {
        Self {
            file_path,
            ast,
            ir: IR::default(),
        }
    }

    pub fn create_from_path(file_path: PathBuf) -> Result<Self, String> {
        let file_path = std::fs::canonicalize(file_path).map_err(|e| e.to_string())?;
        let code = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;

        let tokens = tokenize(&code);
        let tree = parse(&tokens)?;
        let codegen = Codegen::new(file_path, tree);
        Ok(codegen)
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
            NodeKind::Binop(left, op, right) => {
                let lv = self.build(left)?.unwrap();
                let rv = self.build(right)?.unwrap();

                Ok(Some(match op {
                    BinopKind::Add => self.ir.add(lv, rv),
                    BinopKind::Sub => self.ir.sub(lv, rv),
                    BinopKind::Mul => self.ir.mul(lv, rv),
                    BinopKind::Div => self.ir.div(lv, rv),
                }))
            }
            NodeKind::Number(value) => Ok(Some(
                self.ir.constant_signed(node.resolved_type.unwrap(), *value),
            )),
            NodeKind::Import(_) => Ok(None),
            kind => todo!("{:#?}", kind),
        }
    }

    fn resolve_imports(
        &self,
        imports_so_far: Option<&HashSet<PathBuf>>,
    ) -> Result<HashSet<PathBuf>, String> {
        if let Some(imports_so_far) = imports_so_far {
            if imports_so_far.contains(&self.file_path) {
                return Ok(HashSet::new());
            }
        }

        let root = match &self.ast.node {
            NodeKind::Root(v) => v,
            _ => unreachable!(),
        };
        let mut imports = HashSet::<PathBuf>::new();

        for node in root.iter() {
            if let NodeKind::Import(path) = &node.node {
                let path_ext = format!(
                    "{}/{}.drop",
                    self.file_path.parent().unwrap().display(),
                    path.to_string()
                );
                let path_buf = PathBuf::from(&path_ext);
                let path_buf = std::fs::canonicalize(path_buf).map_err(|e| e.to_string())?;
                if !path_buf.exists() {
                    return Err(format!(
                        "(import path {}) file {:?} does not exist",
                        &path, &path_buf
                    ));
                }

                imports.insert(path_buf);
            }
        }

        let mut codegens = Vec::<Codegen>::new();

        for import in imports.iter() {
            codegens.push(Self::create_from_path(import.clone())?);
        }

        for codegen in codegens {
            for import in codegen.resolve_imports(Some(&imports))?.drain() {
                imports.insert(import);
            }
        }

        Ok(imports)
    }

    pub fn generate(&mut self, root: bool) -> Result<(), String> {
        println!("[{:?}] CODEGEN", self.file_path);

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

        // ----------------------------------------------------------

        println!("[{:?}] EMIT", self.file_path);
        let output_path = format!(
            ".dropbuild/{}.c",
            self.file_path
                .display()
                .to_string()
                .replace("/", "_")
                .replace("\\", "_")
        );
        std::fs::write(output_path, code).map_err(|e| e.to_string())?;

        // ----------------------------------------------------------

        if root {
            let mut imports = self.resolve_imports(None)?;
            for import in imports.drain() {
                let mut codegen = Self::create_from_path(import)?;
                codegen.generate(false)?;
            }
        }

        Ok(())
    }
}
