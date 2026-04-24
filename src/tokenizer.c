#include "tokenizer.h"

#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "da.h"
#include "io.h"

char *format_token(Token token) {
    char *name;
    char *inside = NULL;
    switch (token.variant) {
        case Token_Identifier:
            name = "Identifier";
            inside = token.value.identifier;
            break;
        case Token_String:
            name = "String";
            inside = token.value.string;
            break;
        case Token_Integer:
            name = "Integer";
            inside = malloc((strlen(token.value.integer.value) + strlen(token.value.integer.type) + 3) * sizeof(char));
            sprintf(inside, "%s, %s", token.value.integer.value, token.value.integer.type);
            break;
        case Token_Char:
            name = "Char";
            inside = malloc(2 * sizeof(char));
            inside[0] = token.value.character;
            inside[1] = '\0';
            break;
        case Token_PrimType:
            name = "PrimType";
            inside = token.value.prim_type;
            break;
        case Token_FuncType: name = "FuncType"; break;
        case Token_LCurly: name = "LCurly"; break;
        case Token_RCurly: name = "RCurly"; break;
        case Token_LParen: name = "LParen"; break;
        case Token_RParen: name = "RParen"; break;
        case Token_LSquare: name = "LSquare"; break;
        case Token_RSquare: name = "RSquare"; break;
        case Token_Colon: name = "Colon"; break;
        case Token_Semi: name = "Semi"; break;
        case Token_Comma: name = "Comma"; break;
        case Token_SingleEq: name = "SingleEq"; break;
        case Token_DoublePipe: name = "DoublePipe"; break;
        case Token_DoubleAnd: name = "DoubleAnd"; break;
        case Token_Bang: name = "Bang"; break;
        case Token_DoubleEq: name = "DoubleEq"; break;
        case Token_BangEq: name = "BangEq"; break;
        case Token_LessThan: name = "LessThan"; break;
        case Token_MoreThan: name = "MoreThan"; break;
        case Token_LessThanEq: name = "LessThanEq"; break;
        case Token_MoreThanEq: name = "MoreThanEq"; break;
        case Token_Pipe: name = "Pipe"; break;
        case Token_Caret: name = "Caret"; break;
        case Token_Tilde: name = "Tilde"; break;
        case Token_LShift: name = "LShift"; break;
        case Token_RShift: name = "RShift"; break;
        case Token_Plus: name = "Plus"; break;
        case Token_Dash: name = "Dash"; break;
        case Token_Star: name = "Star"; break;
        case Token_Slash: name = "Slash"; break;
        case Token_Percent: name = "Percent"; break;
        case Token_And: name = "And"; break;
        case Token_At: name = "At"; break;
        case Token_Dot: name = "Dot"; break;
        case Token_Arrow: name = "Arrow"; break;
    }

    if (inside) {
        char *full = malloc((strlen(name) + strlen(inside) + 31) * sizeof(char));
        sprintf(full, "%s(%s){%zu, %zu}", name, inside, token.location.row, token.location.col);
        return full;
    } else {
        char *full = malloc((strlen(name) + 29) * sizeof(char));
        sprintf(full, "%s{%zu, %zu}", name, token.location.row, token.location.col);
        return full;
    }
}

void _advance(char *contents, size_t len, size_t *index, size_t *col, char *curr, char *next) {
    *index += 1;
    *col += 1;
    *curr = *next;
    *next = *index + 1 < len ? contents[*index+1] : '\0';
}
#define advance() \
    _advance(file->contents, contents_len, &index, &col, &curr, &next)

Tokens get_tokens(SourceFile *file) {
    DECLARE_DYNAMIC_ARRAY(char, Characters);

    Tokens tokens = {0};

    size_t index = 0;
    size_t row = 0;
    size_t col = 0;

    size_t contents_len = strlen(file->contents);
    char curr = file->contents[index];
    char next = index + 1 < contents_len ? file->contents[index+1] : '\0';
    while (index < contents_len) {
        Token token = {0};

        while (isspace(curr)) {
            if (curr == '\n') {
                advance();
                row++;
                col = 0;
            } else advance();
        }

        token.location.row = row;
        token.location.col = col;
        if (isalpha(curr) || curr == '_') {
            Characters ident = {0};
            while (index < contents_len && (isalnum(curr) || curr == '_')) {
                da_append(&ident, curr);
                advance();
            }
            da_append(&ident, 0);
            if (strcmp(ident.items, "Func") == 0) {
                token.variant = Token_FuncType;
            } else if (is_prim_type(ident.items)) {
                token.variant = Token_PrimType;
                token.value.prim_type = strdup(ident.items);
            } else {
                token.variant = Token_Identifier;
                token.value.string = strdup(ident.items);
            }
            da_free(&ident);
        } else if (isdigit(curr)) {
            Characters number = {0};
            while (index < contents_len && isdigit(curr)) {
                da_append(&number, curr);
                advance();
            }

            token.variant = Token_Integer;
            da_append(&number, 0);
            token.value.integer.value = strdup(number.items);

            switch (curr) {
            case 'u':
            case 'i':
                {
                    Characters type = {0};
                    da_append(&type, curr);
                    advance();
                    while (index < contents_len && isdigit(curr)) {
                        da_append(&type, curr);
                        advance();
                    }
                    if (!is_prim_int(number.items)) {
                        fprintf(stderr, "expected an integer type following an integer literal\n");
                    }
                    da_append(&type, 0);
                    token.value.integer.type = strdup(type.items);
                }
                break;
            default:
                {
                    token.value.integer.type = strdup("i32");
                }
                break;
            }

            da_free(&number);
        } else {
            switch (curr) {
                case '{': token.variant = Token_LCurly; break;
                case '}': token.variant = Token_RCurly; break;
                case '(': token.variant = Token_LParen; break;
                case ')': token.variant = Token_RParen; break;
                case '[': token.variant = Token_LSquare; break;
                case ']': token.variant = Token_RSquare; break;
                case ':': token.variant = Token_Colon; break;
                case ';': token.variant = Token_Semi; break;
                case ',': token.variant = Token_Comma; break;
                case '=': if (next == '=') {
                              token.variant = Token_SingleEq;
                              advance();
                          } else {
                              token.variant = Token_DoubleEq;
                          }
                          break;
                case '!': if (next == '=') {
                              token.variant = Token_BangEq;
                              advance();
                          } else {
                              token.variant = Token_Bang;
                          }
                          break;
                case '<': if (next == '<') {
                              token.variant = Token_LShift;
                              advance();
                          } else if (next == '=') {
                              token.variant = Token_LessThanEq;
                              advance();
                          } else {
                              token.variant = Token_LessThan;
                          }
                          break;
                case '>': if (next == '>') {
                              token.variant = Token_RShift;
                              advance();
                          } else if (next == '=') {
                              token.variant = Token_MoreThanEq;
                              advance();
                          } else {
                              token.variant = Token_MoreThan;
                          }
                          break;
                case '|': if (next == '|') {
                              token.variant = Token_DoublePipe;
                              advance();
                          } else {
                              token.variant = Token_Pipe;
                          }
                          break;
                case '^': token.variant = Token_Caret; break;
                case '~': token.variant = Token_Tilde; break;
                case '+': token.variant = Token_Plus; break;
                case '-': if (next == '>') {
                              token.variant = Token_Arrow;
                              advance();
                          } else {
                              token.variant = Token_Dash;
                          }
                          break;
                case '*': token.variant = Token_Star; break;
                case '/': if (next == '/') {
                              while (index < contents_len && curr != '\n') advance();
                              continue;
                          } else {
                              token.variant = Token_Slash;
                          }
                          break;
                case '%': token.variant = Token_Percent; break;
                case '&': if (next == '&') {
                              token.variant = Token_DoubleAnd;
                              advance();
                          } else {
                              token.variant = Token_And;
                          }
                          break;
                case '@': token.variant = Token_At; break;
                case '.': token.variant = Token_Dot; break;
                case '\0': goto tokenizer_exit;
                default: fprintf(stderr, "unrecognized character '%c'{%d}\n", curr, curr); exit(1);
            }
            advance();
        }

        // printf("(%zu/%zu) %zu: %s\n", index, contents_len, tokens.count-1, format_token(token));
        da_append(&tokens, token);
    }

tokenizer_exit:
    return tokens;
}

bool is_prim_type(char *id) {
    if (strcmp(id, "char") == 0) return true;
    if (strcmp(id, "void") == 0) return true;
    if (is_prim_float(id)) return true;
    if (is_prim_int(id)) return true;
    return false;
}
bool is_prim_float(char *id) {
    if (strcmp(id, "f32") == 0) return true;
    if (strcmp(id, "f64") == 0) return true;
    return false;
}
bool is_prim_int(char *id) {
    if (strcmp(id, "usize") == 0) return true;
    if (strcmp(id, "isize") == 0) return true;
    if (strcmp(id, "u64") == 0) return true;
    if (strcmp(id, "i64") == 0) return true;
    if (strcmp(id, "u32") == 0) return true;
    if (strcmp(id, "i32") == 0) return true;
    if (strcmp(id, "u16") == 0) return true;
    if (strcmp(id, "i16") == 0) return true;
    if (strcmp(id, "u8") == 0) return true;
    if (strcmp(id, "i8") == 0) return true;
    return false;
}
