#include "parser.h"
#include "tokenizer.h"

int main() {
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
  }

  tokens.deinit();

  ast.dump();

  ast.deinit();
}
