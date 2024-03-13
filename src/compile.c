#include "compile.h"
#include "lexer.h"
#include <stdio.h>

char *compile(char *source) {
  Linked_List *token_list = tokenize(source);
  // AST *ast_tree = parse(tokens);
  for (int i = 0; i < token_list->size; i++) {
    Token *t = linked_list_get(token_list, i);
    printf("TOKEN(%d, %s)\n", t->type, t->value);
  }

  return "";
}
