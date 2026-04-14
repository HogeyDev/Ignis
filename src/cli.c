#include "cli.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "da.h"

CliParser cli_parser_from(int argc, char **args) {
    CliParser cli = { 0 };
    cli.args = args;

    for (size_t i = 1; i < argc; i++) { // skip the program name lol
        char *arg = args[i];
        size_t arg_len = strlen(arg);
        if (arg_len && arg[0] == '-') {
            if (arg_len > 1 && arg[1] == '-') {
                char *flag = arg + 2;
                da_append(&cli.flags, flag);
            } else {
                char *option = arg + 1;
                if (i++ < argc) {
                    char *next = args[i];
                    Option opt = {
                        .key = option,
                        .value = next,
                    };
                    da_append(&cli.options, opt);
                } else {
                    fprintf(stderr, "expected argument after `%s`\n", option);
                    exit(1);
                }
            }
        } else {
            da_append(&cli.arguments, arg);
        }
    }

    return cli;
}

bool cli_flag_value(CliParser *clip, char *flag) {
    for (size_t i = 0; i < clip->flags.count; i++) {
        if (strcmp(clip->flags.items[i], flag) == 0)
            return true;
    }
    return false;
}

char *cli_option_value(CliParser *clip, char *option) {
    for (size_t i = 0; i < clip->options.count; i++) {
        Option *curr = clip->options.items + i;
        if (strcmp(curr->key, option) == 0)
          return curr->value;
    }
    return NULL;
}

char *cli_option_fallback(CliParser *clip, char *option, char *fallback) {
    char *value = cli_option_value(clip, option);
    if (!value) {
        return fallback;
    }
    return value;
}

void do_easter_eggs(CliParser *clip) {
    int easter_egg = 0;
    if (cli_flag_value(clip, "nut")) {
        printf("hey hazel!\n");
        easter_egg++;
    }
    if (cli_flag_value(clip, "rizz")) {
        printf("hey iris!\n");
        easter_egg++;
    }
    if (easter_egg) exit(0);
}
