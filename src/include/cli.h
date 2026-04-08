#ifndef CLI_PARSER_H
#define CLI_PARSER_H

#include "ht.h"

struct arguments {
    char **arguments;
    size_t count;
    size_t capacity;
};

typedef struct cli_parser {
    char **args;
    Ht(char *, int) flags; // wanted a hash set but this works LMAO
    Ht(char *, char *) options;
    struct arguments arguments;
} cli_parser;

cli_parser cli_parser_from(int argc, char **args);

#endif
