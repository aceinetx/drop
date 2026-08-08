use crate::codegen::*;
use std::io;

impl Codegen {
    pub(crate) fn get_header_name_for_type_id(&self, t: TypeId) -> String {
        format!("drop_type_{}.h", t)
    }

    pub(crate) fn construct_type_header(&self, t: &Type) -> Option<String> {
        let mut type_dependencies = Vec::<TypeId>::new();

        // Construct a typedef/struct definition based on the type kind
        let typedef = match &t.kind {
            TypeKind::U8() => {
                format!("typedef uint8_t {};", t.c_id)
            }
            TypeKind::I32() => {
                format!("typedef int32_t {};", t.c_id)
            }
            TypeKind::USZ() => {
                format!("typedef size_t {};", t.c_id)
            }
            TypeKind::Pointer(id) => {
                type_dependencies.push(*id);
                let cid = &self.type_table.get(*id).unwrap().c_id;
                format!("typedef {cid} *{};", t.c_id)
            }
            TypeKind::Const(id) => {
                type_dependencies.push(*id);
                let cid = &self.type_table.get(*id).unwrap().c_id;
                format!("typedef const {cid} {};", t.c_id)
            }
            TypeKind::Struct { fields } => {
                let mut code = format!("typedef struct {}{{\n", t.c_id);
                for field in fields {
                    type_dependencies.push(field.1);

                    let field_type_cid = &self.type_table.get(field.1).unwrap().c_id;
                    code += &format!("{field_type_cid} {};\n", field.0);
                }
                code.push('}');
                code.push_str(&t.c_id);
                code.push(';');
                code
            }
            TypeKind::Function { args, return_type } => {
                type_dependencies.append(&mut args.clone());
                type_dependencies.push(*return_type);

                let return_cid = &self.type_table.get(*return_type).unwrap().c_id;
                let args_code = args
                    .clone()
                    .into_iter()
                    .map(|x| self.type_table.get(x).unwrap().c_id.clone())
                    .collect::<Vec<String>>()
                    .join(",");

                format!("typedef {return_cid}(*{})({args_code});", t.c_id)
            }
        };

        // Turn type dependencies into includes
        let dependency_headers: Vec<String> = type_dependencies
            .into_iter()
            .map(|x| self.get_header_name_for_type_id(x))
            .collect();
        let include_lines: Vec<String> = dependency_headers
            .into_iter()
            .map(|x| format!("#include\"{}\"", x))
            .collect();
        let includes = include_lines.join("\n");

        // Construct final header
        let guard = format!("drop_guard_type_{}", t.c_id);

        let mut code = format!(
            r#"
#ifndef {guard}
#define {guard}
#include<stdint.h>
#include<stddef.h>
{includes}
{typedef}
#endif"#
        );
        code.remove(0); // Trim newline

        Some(code)
    }

    pub(crate) fn emit_typedefs(&self) -> io::Result<()> {
        for it in self.type_table.types.iter() {
            if let Some(code) = self.construct_type_header(it.1) {
                let path = format!(".dropbuild/{}", self.get_header_name_for_type_id(*it.0));
                std::fs::write(path, code)?;
            }
        }

        Ok(())
    }
}
