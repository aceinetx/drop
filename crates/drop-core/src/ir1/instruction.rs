use drop_util::table::*;

// extern fn puts (s: *const u8) i32;
//
// fn sixty_nine() i32 {
//   return 69;
// }
//
// fn main () i32 {
//   return puts("Hello, World!");
// }
//
// %0 = typeref(u8);
// %1 = typeconst(%0);
// %2 = typeptr(%1);
// %3 = typeref(i32);
// %4 = funcdef{
//         .is_extern = true,
//         .name = "puts",
//         .args = .[
//                 .( "s",  %2 )
//         ],
//         .return_type = %3,
// };
//
// %5 = number(69);
// %6 = return(%5);
// %7 = typeref(i32);
// %8 = funcdef{
//         .is_extern = true,
//         .name = "sixty_nine",
//         .args = .[],
//         .body = .[%6],
//         .return_type = %7,
// };
//
// %9 = string("Hello, World!");
// %10 = call(%4, %9);
// %11 = return(%10);
// %12 = typeref(i32);
// %13 = funcdef{
//         .is_extern = true,
//         .name = "main",
//         .args = .[],
//         .body = .[%11],
//         .return_type = %12,
// };

#[derive(Debug)]
pub struct FuncDefArg {
    pub name: String,
    pub ty: InstructionId,
}

#[derive(Debug)]
pub struct FuncDef {
    pub is_extern: bool,
    pub name: String,
    pub args: Vec<FuncDefArg>,
    pub body: Option<InstructionId>,
    pub return_type: InstructionId,
}

#[derive(Debug)]
pub enum Instruction {
    TypeRef(String),
    TypeConst(InstructionId),
    TypePtr(InstructionId),
    FuncDef(FuncDef),
    Number(i64),
    String(String),
    Call(InstructionId, Vec<InstructionId>),
    Block(Vec<InstructionId>),
    Return(Option<InstructionId>),
}

pub type InstructionTable = Table<Instruction>;
pub type InstructionId = TableId<Instruction>;
