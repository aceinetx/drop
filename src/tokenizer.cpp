#include "tokenizer.h"
#include <cstdio>

// #region Tokens

drop::Tokens drop::Tokens::init() {
  return {
      List<Token>::init(),
      List<string>::init(),
  };
}

void drop::Tokens::deinit() {
  tokens.deinit();
  for (usize i = 0; i < strings.items.len; i++)
    free(strings.items[i]);
  strings.deinit();
}

void drop::Tokens::dump() {
  puts("Tokens dump:");
  puts("- strings:");
  for (usize i = 0; i < strings.items.len; i++) {
    printf("%5zu = %.*s\n", i, (s32)strings.items[i].len, strings.items[i].ptr);
  }
  puts("- tokens:");
  for (usize i = 0; i < tokens.items.len; i++) {
    printf("%5zu = %zu\n", i, (usize)tokens.items[i].tag);
  }
}

// #endregion

// #region Tokenizer

drop::Tokenizer drop::Tokenizer::init(string code) {
  Tokenizer self = {
      code,
      0,
      Tokens::init(),
  };
  return self;
}

drop::u8 drop::Tokenizer::pch() {
  if (this->pos >= this->code.len)
    return 0;
  return this->code[this->pos];
}

drop::u8 drop::Tokenizer::ch() {
  if (this->pos >= this->code.len)
    return 0;
  return this->code[this->pos++];
}

void drop::Tokenizer::tokenize() {
  Token token;
  do {
    token = _next();
    tokens.tokens.append(token);
  } while (token.tag != TokenType::Eof);
}

drop::Token drop::Tokenizer::_next() {
  u8 c;
  while ((c = pch())) {
    if (isalpha(c)) {
      usize start = this->pos;

      while (isalnum(pch())) {
        ch();
      }

      usize end = this->pos;
      usize len = end - start;

      string s = {code.ptr + start, len};
      s = s.dupe();
      tokens.strings.append(s);

      Token token = {TokenType::Identifier, {tokens.strings.items.len - 1}};
      return token;
    } else if (c == '"') {
      ch();

      usize start = this->pos;

      while (ch() != '"') {
      }

      usize end = this->pos - 1;
      usize len = end - start;

      string s = {code.ptr + start, len};
      s = s.dupe();
      tokens.strings.append(s);

      Token token = {TokenType::String, {tokens.strings.items.len - 1}};
      return token;
    } else if (isspace(c)) {
      ch();
    } else {
      assert(0 && "unknown char");
    }
  }
  return {TokenType::Eof, {0}};
}

// #endregion

drop::Tokens drop::tokenize(string code) {
  auto tokenizer = Tokenizer::init(code);
  tokenizer.tokenize();

  return tokenizer.tokens;
}
