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
  tokens.deinit();
}
