#include "parser.h"
#include <cstdio>

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
      0,
  };
}

void drop::AST::deinit() { nodes.deinit(); }

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
