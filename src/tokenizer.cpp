#include "tokenizer.h"
#include <ctype.h>
#include <stdio.h>

#define X(ident) drop::strlit(#ident),
drop::string drop::token_type_names[] = {XTokenTypes};
#undef X

// #region Tokens

drop::Tokens drop::Tokens::init() {
  return {
      List<Token>::init(),
      List<string>::init(),
  };
}

void drop::Tokens::deinit() {
  list.deinit();
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
  for (usize i = 0; i < list.items.len; i++) {
    auto tag = list.items[i].tag;
    auto data = list.items[i].data;
    auto name = token_type_names[(usize)tag];
    printf("%5zu [%5zu .. %-5zu] = %02zu %-15.*s ", i, list.items[i].position,
           list.items[i].length, (usize)tag, (s32)name.len, name.ptr);

    switch (tag) {
    case TokenType::Identifier:
    case TokenType::String: {
      auto string = strings.items[data.string];
      printf("[string at %zu: %.*s]", data.string, (s32)string.len, string.ptr);
    } break;
    case TokenType::Number:
      printf("%ld", data.number);
      break;
    case TokenType::Unknown:
      putchar(data.ch);
    default:
      break;
    }
    putchar('\n');
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
    tokens.list.append(token);
  } while (token.tag != TokenType::Eof);
}

drop::Token drop::Tokenizer::_next() {
  u8 c;
  while ((c = pch())) {
    if (isalpha(c)) {
      usize start = this->pos;

      while (isident(pch())) {
        ch();
      }

      usize end = this->pos;
      usize len = end - start;

      string s = {code.ptr + start, len};

#define keyword(keyw, type)                                                    \
  if (s.compare(strlit(keyw)))                                                 \
  return {type, {0}, start, len}
      keyword("extern", TokenType::Extern);
      keyword("return", TokenType::Return);
      keyword("fn", TokenType::Fn);
      keyword("const", TokenType::Const);

      size_t i = tokens.strings.append(s.dupe());
      Token token = {TokenType::Identifier, {i}, start, len};
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

      Token token = {TokenType::String,
                     {tokens.strings.items.len - 1},
                     start - 1,
                     len + 1};
      return token;
    } else if (isspace(c)) {
      ch();
#define symbol(sym, type)                                                      \
  }                                                                            \
  else if (c == sym) {                                                         \
    ch();                                                                      \
    return {type, {0}, pos - 1, 1}
      symbol(':', TokenType::Colon);
      symbol(';', TokenType::Semicolon);
      symbol('(', TokenType::Lparen);
      symbol(')', TokenType::Rparen);
      symbol('*', TokenType::Star);
      symbol('{', TokenType::Lbrace);
      symbol('}', TokenType::Rbrace);
      symbol(',', TokenType::Comma);
      symbol('[', TokenType::Lbracket);
      symbol(']', TokenType::Rbracket);
      symbol('+', TokenType::Plus);
      symbol('-', TokenType::Minus);
      symbol('*', TokenType::Mul);
      symbol('/', TokenType::Div);
    } else if (c == '=') {
      ch();
      if (pch() == '>') {
        ch();
        return {TokenType::FatArrow, {0}, pos - 2, 2};
      } else {
        return {TokenType::Eq, {0}, pos - 1, 1};
      }
    } else if (isdigit(c)) {
      usize start = pos;

      s64 number = 0;

      while (isdigit(pch())) {
        number *= 10;
        number += ch() - '0';
      }

      usize len = pos - start;

      return Token::make_number(number, start, len);
    } else {
      ch();
      return Token::make_unknown(c, pos - 1, 1);
    }
  }
  return {TokenType::Eof, {0}, pos, 1};
}

// #endregion

drop::Tokens drop::tokenize(string code) {
  auto tokenizer = Tokenizer::init(code);
  tokenizer.tokenize();

  return tokenizer.tokens;
}
