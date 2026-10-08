#pragma once
#include "tokenizer.h"

namespace drop {
#define XAstNodeTypes                                                          \
  X(Root)                                                                      \
  X(FuncDef)

#define X(ident) ident,
enum class ASTNodeType : u8 { XAstNodeTypes };
#undef X

extern string ast_node_type_names[];

struct NodeFuncDef {
  bool is_extern;

  // Index of strings AST field
  usize name;

  // Index of nodes AST field
  usize return_type;

  // Index of nodes AST field
  usize body;
};

union ASTNodeData {
  // Index of node_arrays list of AST
  usize root;

  NodeFuncDef funcdef;
};

struct ASTNode {
  ASTNodeType type;
  ASTNodeData data;

  void dump(usize indent);
};

struct AST {
  List<ASTNode> nodes;
  List<List<usize>> node_arrays;
  List<string> strings;
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
