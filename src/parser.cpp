#include "parser.h"
#include <stdio.h>

#define X(ident) drop::strlit(#ident),
drop::string drop::ast_node_type_names[] = {XAstNodeTypes};
#undef X

// #region ASTNode

void drop::ASTNode::dump(usize indent) {
  for (usize i = 0; i < indent; i++)
    putchar('\t');
  auto name = ast_node_type_names[(usize)type];
  printf("- %.*s\n", (s32)name.len, name.ptr);
}

// #endregion

// #region AST

drop::AST drop::AST::init() {
  return {
      List<ASTNode>::init(),
      List<List<usize>>::init(),
      0,
  };
}

void drop::AST::deinit() {
  for (usize i = 0; i < blocks.items.len; i++) {
    blocks.items[i].deinit();
  }
  blocks.deinit();
  nodes.deinit();
}

void drop::AST::dump() {
  auto node = nodes.items[root];
  node.dump(0);
}

// #endregion

// #region Parser

drop::Parser drop::Parser::init(const Tokens *const tokens) {
  return Parser{
      tokens,
      AST::init(),
  };
}

void drop::Parser::parse() {
  usize i = ast.nodes.append({ASTNodeType::Root, {}});
  ast.root = i;
}

// #endregion

drop::AST drop::parse(const Tokens *const tokens) {
  auto parser = Parser::init(tokens);
  parser.parse();
  return parser.ast;
}
