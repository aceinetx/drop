#pragma once
#include "tokenizer.h"

namespace drop {
#define XAstNodeTypes X(Root)

#define X(ident) ident,
enum class ASTNodeType : u8 { XAstNodeTypes };
#undef X

extern string ast_node_type_names[];

union ASTNodeData {};

struct ASTNode {
  ASTNodeType type;
  ASTNodeData data;

  void dump(usize indent);
};

struct AST {
  List<ASTNode> nodes;
  List<List<usize>> blocks;
  usize root;

  static AST init();

  void deinit();

  void dump();
};

struct Parser {
  const Tokens *const tokens;
  AST ast;

  static Parser init(const Tokens *tokens);
  void parse();
};

AST parse(const Tokens *const tokens);
} // namespace drop
