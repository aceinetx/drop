#include "tokenizer.h"

int main() {
  auto tokens = drop::tokenize(drop::strlit("main \"hello\""));
  tokens.dump();
  tokens.deinit();
}
