#include "parser.h"
#include "string_format_arg.h"
#include <stdio.h>

#define X(ident) drop::strlit(#ident),
drop::string drop::ast_node_type_names[] = {XAstNodeTypes};
#undef X

// #region ASTNode

static inline void print_indent(drop::usize indent) {
  for (drop::usize i = 0; i < indent; i++)
    putchar('\t');
}

void drop::ASTNode::dump(AST *ast, usize indent) {
  print_indent(indent);

  auto name = ast_node_type_names[(usize)type];
  printf("- " sv_fmt "\n", sv_arg(name));

  switch (type) {
  case ASTNodeType::Root: {
    auto block = &ast->node_arrays.items[data.node_array];
    for (usize i = 0; i < block->items.len; i++) {
      auto node = &ast->nodes.items[block->items[i]];
      node->dump(ast, indent + 1);
    }
  } break;
  case ASTNodeType::TypeRef: {
    print_indent(indent + 1);
    printf(sv_fmt "\n", sv_arg(ast->strings.items[data.string]));
  } break;
  case ASTNodeType::FuncDef: {
    auto funcdef = &data.funcdef;
    print_indent(indent + 1);
    printf("- is_extern: %s\n", drop_bool_fmt(funcdef->is_extern));
    print_indent(indent + 1);
    printf("- name: " sv_fmt "\n", sv_arg(ast->strings.items[funcdef->name]));
    print_indent(indent + 1);
    printf("- return_type\n");
    ast->nodes.items[funcdef->return_type].dump(ast, indent + 2);
    print_indent(indent + 1);
    printf("- body\n");
    ast->nodes.items[funcdef->body].dump(ast, indent + 2);
  } break;
  case ASTNodeType::Block: {
    auto block = &ast->node_arrays.items[data.node_array];
    for (usize i = 0; i < block->items.len; i++) {
      auto node = &ast->nodes.items[block->items[i]];
      node->dump(ast, indent + 1);
    }
  } break;
  }
}

// #endregion

// #region AST

drop::AST drop::AST::init() {
  return {
      List<ASTNode>::init(),
      List<List<usize>>::init(),
      List<string>::init(),
      0,
  };
}

void drop::AST::deinit() {
  for (usize i = 0; i < strings.items.len; i++) {
    free(strings.items[i]);
  }
  strings.deinit();

  for (usize i = 0; i < node_arrays.items.len; i++) {
    node_arrays.items[i].deinit();
  }
  node_arrays.deinit();

  nodes.deinit();
}

void drop::AST::dump() {
  auto node = nodes.items[root];
  node.dump(this, 0);
}

// #endregion

// #region Parser

drop::Parser drop::Parser::init(const Tokens *const tokens) {
  return Parser{
      tokens,
      AST::init(),
      0,
  };
}

namespace drop {
static Token peek(Parser *self) {
  return self->tokens->list.items[self->token_index];
}

static Token next(Parser *self) {
  auto token = peek(self);
  self->token_index++;
  return token;
}

static Token expect(Parser *self, ParserDiagnostics *diagnostics,
                    TokenType type) {
  const auto token = next(self);
  if (token.tag == type) {
    return token;
  }

  auto expected_name = token_type_names[(usize)type];
  auto found_name = token_type_names[(usize)token.tag];
  sprintf((char *)diagnostics->message,
          "expected " sv_fmt ", but found " sv_fmt, sv_arg(expected_name),
          sv_arg(found_name));
  diagnostics->position = token.position;

  return {TokenType::Eof, {0}, 0, 0};
}

static usize parse_type(Parser *self, ParserDiagnostics *diagnostics) {
  const auto first = next(self);
  switch (first.tag) {
  case TokenType::Identifier: {
    ASTNode node = {ASTNodeType::TypeRef, {0}};
    auto string = self->tokens->strings.items[first.data.string].dupe();
    node.data.string = self->ast.strings.append(string);
    return self->ast.nodes.append(node);
  } break;
  case TokenType::Star: {
    ASTNode node = {ASTNodeType::TypePtr, {0}};
    node.data.node = parse_type(self, diagnostics);
    return self->ast.nodes.append(node);
  } break;
  case TokenType::Const: {
    ASTNode node = {ASTNodeType::TypeConst, {0}};
    node.data.node = parse_type(self, diagnostics);
    return self->ast.nodes.append(node);
  } break;
  default: {
    auto name = token_type_names[(usize)first.tag];
    diagnostics->position = first.position;
    sprintf((char *)diagnostics->message,
            "expected Identifier or Star or Const, but found " sv_fmt,
            sv_arg(name));
    return 0;
  }
  }
}

static usize parse_block(Parser *self, ParserDiagnostics *diagnostics) {
  if (!expect(self, diagnostics, TokenType::Lbrace))
    return 0;
  if (!expect(self, diagnostics, TokenType::Rbrace))
    return 0;

  ASTNode node = {ASTNodeType::Block, {0}};

  auto list = self->ast.node_arrays.append(List<usize>::init());
  node.data.node_array = list;

  return self->ast.nodes.append(node);
}

static usize parse_func(Parser *self, ParserDiagnostics *diagnostics) {
  const auto first = next(self);
  ASTNode node = {ASTNodeType::FuncDef, {0}};
  memset(&node.data, 0, sizeof node.data);

  if (first.tag == TokenType::Extern) {
    node.data.funcdef.is_extern = true;

    if (!expect(self, diagnostics, TokenType::Fn))
      return 0;
  } else if (first.tag == TokenType::Fn) {
  } else {
    auto name = token_type_names[(usize)first.tag];
    diagnostics->position = first.position;
    sprintf((char *)diagnostics->message,
            "expected Extern or Fn, but found " sv_fmt, sv_arg(name));
    return 0;
  }

  auto name = expect(self, diagnostics, TokenType::Identifier);
  if (!name)
    return 0;
  node.data.funcdef.name = self->ast.strings.append(
      self->tokens->strings.items[name.data.string].dupe());

  // TODO args parsing
  if (!expect(self, diagnostics, TokenType::Lparen))
    return 0;
  if (!expect(self, diagnostics, TokenType::Rparen))
    return 0;

  auto type = parse_type(self, diagnostics);
  if (!type)
    return 0;
  node.data.funcdef.return_type = type;

  auto body = parse_block(self, diagnostics);
  if (!body)
    return 0;
  node.data.funcdef.body = body;

  usize node_index = self->ast.nodes.append(node);
  return node_index;
}

static usize parse_tld(Parser *self, ParserDiagnostics *diagnostics) {
  return parse_func(self, diagnostics);
}
} // namespace drop

bool drop::Parser::parse(ParserDiagnostics *diagnostics) {
  memset(diagnostics, 0, sizeof *diagnostics);

  usize i = ast.nodes.append(
      {ASTNodeType::Root, {ast.node_arrays.append(List<usize>::init())}});
  ast.root = i;

  usize list_id = ast.nodes.items[ast.root].data.node_array;

  while (peek(this).tag != TokenType::Eof) {
    usize index;
    if (!(index = parse_tld(this, diagnostics)))
      return false;

    ast.node_arrays.items[list_id].append(index);
  }

  return true;
}

// #endregion

bool drop::parse(const Tokens *const tokens, AST *ast,
                 ParserDiagnostics *diagnostics) {
  auto parser = Parser::init(tokens);

  bool ok = parser.parse(diagnostics);

  *ast = parser.ast;

  return ok;
}
