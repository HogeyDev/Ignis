#ifndef LEXER_H
#define LEXER_H

#include "array.h"
typedef enum {
  TOKEN_EOF,
  TOKEN_IMPORT,
  TOKEN_ID,
  TOKEN_FN,
  TOKEN_FOR,
  TOKEN_ASM,
  TOKEN_SEMI,
  TOKEN_COLON,
  TOKEN_EQUALS,
  TOKEN_LPAREN,
  TOKEN_RPAREN,
  TOKEN_LBRACE,
  TOKEN_RBRACE,
  TOKEN_LT,
  TOKEN_GT,
  TOKEN_LTE,
  TOKEN_GTE,
  TOKEN_IF,
  TOKEN_LET,
  TOKEN_INTEGER,
  TOKEN_STRING,
  TOKEN_PLUS,
  TOKEN_MINUS,
  TOKEN_STAR,
  TOKEN_SLASH,
  TOKEN_INCREMENT,
  TOKEN_DECREMENT,
  TOKEN_COMMA,
} Token_Type;

typedef struct {
  Token_Type type;
  char *value;
} Token;

Token *new_empty_token(void);
Token *new_token(Token_Type type, char *value);

typedef struct {
  char *source;
  int index;
} Tokenizer;

Tokenizer *new_tokenizer(char *source);

void tokenizer_skip_whitespace(Tokenizer *tokenizer);
char tokenizer_peek(Tokenizer *tokenizer, int offset);
Token *tokenizer_parse_identifier(Tokenizer *tokenizer);
Token *tokenizer_parse_number(Tokenizer *tokenizer);
Token *tokenizer_parse_string(Tokenizer *tokenizer);
Token *tokenizer_skip_and_return(Tokenizer *tokenizer, Token_Type type);
Token *tokenizer_skip_and_return_value(Tokenizer *tokenizer, Token_Type type,
                                       char *value);
Token *tokenizer_let_next_token(Tokenizer *tokenizer);
Linked_List *tokenize(char *source);

#endif // !LEXER_H
