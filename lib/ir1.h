#pragma once
#include "base.h"
#include "hash_map.h"
#include "list.h"
#include "parser.h"
#include "str.h"

namespace drop {
/*
extern fn puts (s: *const u8) i32;

fn sixty_nine() i32 {
  return 69;
}

fn main () i32 {
  return puts("Hello, World!");
}

%0 = typeref(u8);
%1 = typeconst(%0);
%2 = typeptr(%1);
%3 = typeref(i32);
%4 = funcdef{
        .is_extern = true,
        .name = "puts",
        .args = .[
                .( "s",  %2 )
        ],
        .return_type = %3,
};

%5 = number(69);
%6 = return(%5);
%7 = typeref(i32);
%8 = funcdef{
        .is_extern = true,
        .name = "sixty_nine",
        .args = .[],
        .body = .[%6],
        .return_type = %7,
};

%9 = string("Hello, World!");
%10 = call(%4, %9);
%11 = return(%10);
%12 = typeref(i32);
%13 = funcdef{
        .is_extern = true,
        .name = "main",
        .args = .[],
        .body = .[%11],
        .return_type = %12,
};
 */

#define XInstructionTypes                                                      \
  X(TypeRef)                                                                   \
  X(TypeConst)                                                                 \
  X(TypePtr)                                                                   \
  X(FuncDef)                                                                   \
  X(Number)                                                                    \
  X(String)                                                                    \
  X(Call)                                                                      \
  X(Return)

#define X(ident) ident,
enum InstructionType { XInstructionTypes };
#undef X

extern string instruction_type_names[];

union InstructionData {
  // Index of strings field of IR1
  usize string;

  s64 number;
};

struct Instruction {
  InstructionType tag;
  InstructionData data;
};

struct IR1FuncDefArg {
  // Index of strings field of IR1
  string name;

  // Index of insts field of IR1
  usize insts;
};

struct IR1FuncDef {
  bool is_extern;

  // Index of strings field of IR1
  usize name;

  List<IR1FuncDefArg> args;

  // Index of lists_of_insts field of IR1
  usize body;
};

struct IR1 {
  List<Instruction> insts;

  List<List<usize>> lists_of_insts;

  List<IR1FuncDef> funcdefs;

  List<string> strings;

  static IR1 init();

  void deinit();

  void dump();
};

struct IR1GenDiagnostics {
  u8 message[512];
  usize position;
  usize position_length;
};

using Scope = HashMap<string, usize, slice_hash, slice_compare>;
struct IR1FunctionContext {
  List<Scope> scopes;
};

struct IR1Gen {
  IR1 ir;
  AST *ast;
  List<Scope> scopes;

  static IR1Gen init(AST *ast);
};

bool generate_ir1(AST *ast, IR1 *ir1, IR1GenDiagnostics *diag);
} // namespace drop
