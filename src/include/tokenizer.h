#ifndef TOKENIZER_H
#define TOKENIZER_H

#include "io.h"
#include <stdbool.h>
#include <stddef.h>

typedef struct {
    enum {
        Token_Identifier,
        Token_String,
        Token_Integer,
        Token_Char,

        Token_PrimType,
        Token_FuncType,

        Token_LCurly,
        Token_RCurly,
        Token_LParen,
        Token_RParen,
        Token_LSquare,
        Token_RSquare,

        Token_Colon,
        Token_Semi,
        Token_Comma,

        Token_SingleEq,
        Token_DoublePipe,
        Token_DoubleAnd,
        Token_Bang,
        Token_DoubleEq,
        Token_BangEq,
        Token_LessThan,
        Token_MoreThan,
        Token_LessThanEq,
        Token_MoreThanEq,
        Token_Pipe,
        Token_Caret,
        Token_Tilde,
        Token_LShift,
        Token_RShift,
        Token_Plus,
        Token_Dash,
        Token_Star,
        Token_Slash,
        Token_Percent,
        Token_And,
        Token_At,

        Token_Dot,
        Token_Arrow,
    } variant;
    union {
        char *identifier;
        char *string;
        struct {
            char *value;
            char *type;
        } integer;
        char character;
        char *prim_type;
    } value;

    struct {
        size_t row;
        size_t col;
    } location;
} Token;

typedef struct {
    Token *items;
    size_t count;
    size_t capacity;
} Tokens;

char *format_token(Token token);

Tokens get_tokens(SourceFile *file);

bool is_prim_type(char *id);
bool is_prim_float(char *id);
bool is_prim_int(char *id);

#endif
