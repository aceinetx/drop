#pragma once

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

union TokenData {};

struct Token {};

struct Tokenizer {};
} // namespace drop
