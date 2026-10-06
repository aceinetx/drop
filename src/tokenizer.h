#pragma once
#include "base.h"
#include "list.h"

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
  string string;
  s64 number;
};

struct Token {
  TokenType tag;
  TokenData data;
};

struct Tokenizer {
  string code;
  size_t pos;
  List<Token> tokens;

  static Tokenizer init(string code) {
    Tokenizer self = {
        code,
        0,
        List<Token>::init(),
    };
    return self;
  }

  Token next() {}
};

List<Token> tokenize(string code) {
  auto tokenizer = Tokenizer::init(code);
  Token token;
  return tokenizer;
}
} // namespace drop
