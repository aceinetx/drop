#pragma once
#include "list.h"
#include "string.h"

namespace drop {
#define XTokenTypes                                                            \
  X(Eof)                                                                       \
  X(Identifier)                                                                \
  X(String)                                                                    \
  X(Colon)                                                                     \
  X(Semicolon)                                                                 \
  X(Lparen)                                                                    \
  X(Rparen)                                                                    \
  X(Star)                                                                      \
  X(Lbrace)                                                                    \
  X(Rbrace)                                                                    \
  X(Extern)                                                                    \
  X(Return)                                                                    \
  X(Fn)                                                                        \
  X(Comma)                                                                     \
  X(Const)                                                                     \
  X(Lbracket)                                                                  \
  X(Rbracket)                                                                  \
  X(Number)                                                                    \
  X(Plus)                                                                      \
  X(Minus)                                                                     \
  X(Mul)                                                                       \
  X(Div)                                                                       \
  X(Directive)                                                                 \
  X(FatArrow)                                                                  \
  X(Eq)

#define X(ident) ident,
enum class TokenType { XTokenTypes };
#undef X

extern string token_type_names[];

union TokenData {
  usize string;
  s64 number;
};

struct Token {
  TokenType tag;
  TokenData data;
};

struct Tokens {
  List<Token> tokens;
  List<string> strings;

  static Tokens init();

  void deinit();

  void dump();
};

struct Tokenizer {
  string code;
  size_t pos;
  Tokens tokens;

  static Tokenizer init(string code);

  u8 pch();

  u8 ch();

  void tokenize();

  Token _next();
};

Tokens tokenize(string code);
} // namespace drop
