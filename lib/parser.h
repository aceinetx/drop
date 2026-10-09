#pragma once
#include "pair.h"
#include "tokenizer.h"

namespace drop {
#define XAstNodeTypes                                                          \
  X(Root /* Uses the node_array data union field */)                           \
  X(FuncDef)                                                                   \
  X(TypeRef /* Uses the string data union field */)                            \
  X(TypePtr /* Uses the node data union field */)                              \
  X(TypeConst /* Uses the node data union field */)                            \
  X(Block /* Uses the node_array data union field */)

#define X(ident) ident,
enum class ASTNodeType : u8 { XAstNodeTypes };
#undef X

extern string ast_node_type_names[];

struct NodeFuncDefArg {
  // Index of strings AST field
  usize name;

  // Index of nodes AST field
  usize type;
};

struct NodeFuncDef {
  bool is_extern;

  // Index of strings AST field
  usize name;

  // Index of nodes AST field
  usize return_type;

  // Index of nodes AST field
  usize body;

  List<NodeFuncDefArg> args;

  void deinit();
};

union ASTNodeData {
  // Index of node_arrays list of AST
  usize node_array;

  // Index of string list of AST
  usize string;

  // Index of funcdefs list of AST
  usize funcdef;

  // Index of nodes list of AST
  usize node;
};

struct AST;

struct ASTNode {
  ASTNodeType type;
  ASTNodeData data;

  void dump(AST *ast, usize indent);
};

struct AST {
  List<ASTNode> nodes;
  List<List<usize>> node_arrays;
  List<string> strings;
  List<NodeFuncDef> funcdefs;

  usize root;

  static AST init();

  void deinit();

  void dump();
};

struct ParserDiagnostics {
  usize position;
  u8 message[512];
};

struct Parser {
  const Tokens *const tokens;
  AST ast;
  usize token_index;

  static Parser init(const Tokens *tokens);

  bool parse(ParserDiagnostics *diagnostics);
};

bool parse(const Tokens *const tokens, AST *ast,
           ParserDiagnostics *diagnostics);
} // namespace drop
