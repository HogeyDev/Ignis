#include "parser.h"
#include <string.h>

int ident(Token t, char *v) {
    return (t.variant == Token_Identifier && strcmp(t.value.identifier, v) == 0);
}

#define tcurr parser.tokens->items[parser.index]

Ast *parse_tokens(Tokens *tokens) {
    Parser parser = {0};

    AstArray program = {0};
    while (parser.index < tokens->count) {
        switch (tcurr.variant) {
        case Token_Identifier: {
            break;
        } break;
        }
    }

    return 0;
}
