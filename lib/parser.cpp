#include "parser.h"
#include "string_format_arg.h"
#include <stdio.h>

#define X(ident) drop::strlit(#ident),
drop::string drop::ast_node_type_names[] = {XAstNodeTypes};
#undef X

// #region ASTNode

void drop::ASTNode::dump(usize indent) {
  for (usize i = 0; i < indent; i++)
    putchar('\t');
  auto name = ast_node_type_names[(usize)type];
  printf("- " sv_fmt "\n", sv_arg(name));
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
  for (usize i = 0; i < node_arrays.items.len; i++) {
    node_arrays.items[i].deinit();
  }
  node_arrays.deinit();
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

static usize parse_func(Parser *self, ParserDiagnostics *diagnostics) {
  const auto first = next(self);
  bool is_extern = false;

  if (first.tag == TokenType::Extern) {
    if (!expect(self, diagnostics, TokenType::Fn))
      return 0;

    is_extern = true;
  } else if (first.tag == TokenType::Fn) {
    drop_trap();
  } else {
    auto name = token_type_names[(usize)first.tag];
    sprintf((char *)diagnostics->message,
            "expected Extern or Fn, but found " sv_fmt, sv_arg(name));
    return 0;
  }

  return 1;
}

static bool parse_tld(Parser *self, ParserDiagnostics *diagnostics) {
  return !!parse_func(self, diagnostics);
}
} // namespace drop

bool drop::Parser::parse(ParserDiagnostics *diagnostics) {
  memset(diagnostics, 0, sizeof *diagnostics);

  usize i = ast.nodes.append(
      {ASTNodeType::Root, {ast.node_arrays.append(List<usize>::init())}});
  ast.root = i;

  while (peek(this).tag != TokenType::Eof) {
    usize index;
    if (!(index = parse_tld(this, diagnostics)))
      return false;

    usize list_id = ast.nodes.items[ast.root].data.root;
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
