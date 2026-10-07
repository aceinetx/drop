#pragma once
#include "list.h"
#include "string.h"

namespace drop {
#define XTokenTypes                                                            \
  X(Eof)                                                                       \
  X(Unknown)                                                                   \
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
enum class TokenType : u8 { XTokenTypes };
#undef X

extern string token_type_names[];

union TokenData {
  // Index of strings field of Tokens structure
  usize string;

  s64 number;

  u8 ch;
};

struct Token {
  TokenType tag;
  TokenData data;
  usize position;
  usize length;

  static Token make_number(s64 number, usize position, usize length) {
    Token token = {TokenType::Number, {0}, position, length};
    token.data.number = number;
    return token;
  }

  static Token make_unknown(u8 ch, usize position, usize length) {
    Token token = {TokenType::Unknown, {0}, position, length};
    token.data.ch = ch;
    return token;
  }
};

struct Tokens {
  List<Token> list;
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
