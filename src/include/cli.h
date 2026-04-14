#ifndef CLI_PARSER_H
#define CLI_PARSER_H

#include <stddef.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
    char **items;
    size_t count;
    size_t capacity;
} Flags;

typedef struct {
    char *key;
    char *value;
} Option;
typedef struct {
    Option *items;
    size_t count;
    size_t capacity;
} Options;

typedef struct {
    char **items;
    size_t count;
    size_t capacity;
} Arguments;

typedef struct {
    char **args;
    Flags flags;
    Options options;
    Arguments arguments;
} CliParser;

CliParser cli_parser_from(int argc, char **args);
bool cli_flag_value(CliParser *clip, char *flag);
char *cli_option_value(CliParser *clip, char *option);
char *cli_option_fallback(CliParser *clip, char *option, char *fallback);
void do_easter_eggs(CliParser *clip);

#endif
