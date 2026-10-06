#pragma once
#include "list.h"
#include "string.h"

namespace drop {
enum class TokenType {
  Eof,
  Identifier,
  String,
  Colon,
  Semicolon,
  Lparen,
  Rparen,
  Star,
  Lbrace,
  Rbrace,
  Extern,
  Return,
  Fn,
  Comma,
  Const,
  Lbracket,
  Rbracket,
  Number,
  Plus,
  Minus,
  Mul,
  Div,
  Directive,
  FatArrow,
  Eq,
};

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
