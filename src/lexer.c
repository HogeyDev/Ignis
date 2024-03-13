#include "lexer.h"
#include "array.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Token *new_empty_token(void) {
  Token *t = malloc(sizeof(Token));

  t->value = malloc(0);

  return t;
}

Token *new_token(Token_Type type, char *value) {
  Token *t = malloc(sizeof(Token));

  t->type = type;
  t->value = malloc(strlen(value) + 1);
  strcpy(t->value, value);

  return t;
}

Tokenizer *new_tokenizer(char *source) {
  Tokenizer *tokenizer = malloc(sizeof(Tokenizer));

  tokenizer->source = malloc(strlen(source) + 1);
  strcpy(tokenizer->source, source);
  tokenizer->index = 0;

  return tokenizer;
}

void tokenizer_skip_whitespace(Tokenizer *tokenizer) {
  while (isspace(tokenizer->source[tokenizer->index])) {
    tokenizer->index++;
  }
}

char tokenizer_peek(Tokenizer *tokenizer, int offset) {
  return tokenizer->source[tokenizer->index + offset];
}

Token *tokenizer_parse_identifier(Tokenizer *tokenizer) {
  Token *t = new_token(TOKEN_ID, "");

  while (isalnum(tokenizer->source[tokenizer->index])) {
    t->value = realloc(t->value, strlen(t->value) + 2);
    strcat(t->value, (char[]){tokenizer->source[tokenizer->index], 0});

    tokenizer->index++;
  }

  if (strcmp(t->value, "import") == 0) {
    t->type = TOKEN_IMPORT;
  } else if (strcmp(t->value, "fn") == 0) {
    t->type = TOKEN_FN;
  } else if (strcmp(t->value, "if") == 0) {
    t->type = TOKEN_IF;
  } else if (strcmp(t->value, "let") == 0) {
    t->type = TOKEN_LET;
  } else if (strcmp(t->value, "for") == 0) {
    t->type = TOKEN_FOR;
  } else if (strcmp(t->value, "asm") == 0) {
    t->type = TOKEN_ASM;
    tokenizer_skip_whitespace(tokenizer);
    if (tokenizer->source[tokenizer->index] != '{') {
      fprintf(stderr, "Expected '{' after asm to open assembly block");
      exit(1);
    }
    tokenizer->index++;

    free(t->value);
    t->value = malloc(1);
    *t->value = 0; // null terminator
    while (tokenizer->source[tokenizer->index] != '}') {
      t->value = realloc(t->value, strlen(t->value) + 2);
      strcat(t->value, (char[]){tokenizer->source[tokenizer->index], 0});

      tokenizer->index++;
    }
    tokenizer_skip_whitespace(tokenizer);
    if (tokenizer->source[tokenizer->index] != '}') {
      fprintf(stderr, "Expected '}' to end assembly block");
      exit(1);
    }
    tokenizer->index++;
  }

  return t;
}

Token *tokenizer_parse_number(Tokenizer *tokenizer) {
  Token *t = new_token(TOKEN_INTEGER, "");

  while (isdigit(tokenizer->source[tokenizer->index])) {
    t->value = realloc(t->value, strlen(t->value) + 2);
    strcat(t->value, (char[]){tokenizer->source[tokenizer->index], 0});

    tokenizer->index++;
  }

  return t;
}

Token *tokenizer_parse_string(Tokenizer *tokenizer) {
  Token *t = new_token(TOKEN_STRING, "");

  tokenizer->index++;
  while (tokenizer->source[tokenizer->index] != '\"') {
    t->value = realloc(t->value, strlen(t->value) + 2);
    strcat(t->value, (char[]){tokenizer->source[tokenizer->index], 0});

    tokenizer->index++;
  }
  tokenizer->index++;

  return t;
}

Token *tokenizer_skip_and_return(Tokenizer *tokenizer, Token_Type type) {
  char *value = (char[]){tokenizer->source[tokenizer->index++], 0};

  return new_token(type, value);
}

Token *tokenizer_skip_and_return_value(Tokenizer *tokenizer, Token_Type type,
                                       char *value) {
  tokenizer->index += strlen(value);

  return new_token(type, value);
}

Token *get_next_token(Tokenizer *tokenizer) {
  tokenizer_skip_whitespace(tokenizer);
  if (isalpha(tokenizer->source[tokenizer->index])) {
    return tokenizer_parse_identifier(tokenizer);
  }
  if (isdigit(tokenizer->source[tokenizer->index])) {
    return tokenizer_parse_number(tokenizer);
  }
  if (tokenizer->source[tokenizer->index] == '\"') {
    return tokenizer_parse_string(tokenizer);
  }
  switch (tokenizer->source[tokenizer->index]) {
  case ';':
    return tokenizer_skip_and_return(tokenizer, TOKEN_SEMI);
  case ':':
    return tokenizer_skip_and_return(tokenizer, TOKEN_COLON);
  case '=': // TODO: add == check
    return tokenizer_skip_and_return(tokenizer, TOKEN_EQUALS);
  case '(':
    return tokenizer_skip_and_return(tokenizer, TOKEN_LPAREN);
  case ')':
    return tokenizer_skip_and_return(tokenizer, TOKEN_RPAREN);
  case '{':
    return tokenizer_skip_and_return(tokenizer, TOKEN_LBRACE);
  case '}':
    return tokenizer_skip_and_return(tokenizer, TOKEN_RBRACE);
  case '<':
    if (tokenizer_peek(tokenizer, 1) == '=') {
      return tokenizer_skip_and_return_value(tokenizer, TOKEN_LTE, "<=");
    }
    return tokenizer_skip_and_return(tokenizer, TOKEN_LT);
  case '>':
    if (tokenizer_peek(tokenizer, 1) == '=') {
      return tokenizer_skip_and_return_value(tokenizer, TOKEN_GTE, ">=");
    }
    return tokenizer_skip_and_return(tokenizer, TOKEN_GT);
  case '+':
    if (tokenizer_peek(tokenizer, 1) == '+') {
      return tokenizer_skip_and_return_value(tokenizer, TOKEN_INCREMENT, "++");
    }
    return tokenizer_skip_and_return(tokenizer, TOKEN_PLUS);
  case '-':
    if (tokenizer_peek(tokenizer, 1) == '-') {
      return tokenizer_skip_and_return_value(tokenizer, TOKEN_DECREMENT, "--");
    }
    return tokenizer_skip_and_return(tokenizer, TOKEN_MINUS);
  case ',':
    return tokenizer_skip_and_return(tokenizer, TOKEN_COMMA);
  }

  return new_token(TOKEN_EOF, "");
}

Linked_List *tokenize(char *source) {
  Tokenizer *tokenizer = new_tokenizer(source);
  Linked_List *token_list = new_linked_list();

  Token *current_token;
  while ((current_token = get_next_token(tokenizer))->type != TOKEN_EOF) {
    linked_list_push(token_list, current_token);
  }
  linked_list_push(token_list, new_token(TOKEN_EOF, "\0"));

  return token_list;
}
