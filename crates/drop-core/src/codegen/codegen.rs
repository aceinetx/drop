use std::io;

use crate::codegen::TypeTable;
use crate::parser::Node;

pub struct Codegen {
    pub(crate) ast: Node,
    pub(crate) type_table: TypeTable,
}

impl Codegen {
    pub fn new(ast: Node) -> Self {
        let mut type_table = TypeTable::default();
        type_table.add_builtins();
        Self { ast, type_table }
    }

    pub fn generate(&mut self) -> Result<(), String> {
        if let Err(e) = std::fs::create_dir(".dropbuild")
            && e.kind() != io::ErrorKind::AlreadyExists
        {
            return Err(e.to_string());
        }

        self.resolve_types_root()?;
        self.emit_typedefs().map_err(|e| e.to_string())?;

        Ok(())
    }
}
