#include "base.h"
#include "ir1.h"
#include "parser.h"
#include "tokenizer.h"
#include <stdio.h>

void test() {
  auto code = drop::strlit(R"(
extern fn puts (s: *const u8) i32;

fn sixty_nine() i32 {
  return 69;
}

fn main () i32 {
  return puts("Hello, World!");
}
)");

  auto tokens = drop::tokenize(code);
  tokens.dump();

  drop::AST ast;
  {
    drop::ParserDiagnostics diag;
    if (!drop::parse(&tokens, &ast, &diag)) {
      printf("parse error at position %zu: %s\n", diag.position, diag.message);
      printf("%c\n", code[diag.position]);
      printf("%.*s\n", (drop::s32)diag.position + 1, code.ptr);

      drop_trap();
    }

    tokens.deinit();
  }

  ast.dump();

  drop::IR1 ir1;
  {
    drop::IR1GenDiagnostics diag;
    if (!drop::generate_ir1(&ast, &ir1, &diag)) {
      printf("ir1 gen error at position %zu..%zu: %s\n", diag.position,
             diag.position_length, diag.message);
      printf("%.*s\n", (drop::s32)diag.position_length,
             code.ptr + diag.position);

      drop_trap();
    }

    ast.deinit();
  }

  ir1.dump();

  ir1.deinit();

  printf("%.*s", 15, code.ptr + 106);
}
