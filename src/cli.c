#include "cli.h"
#include "ht.h"

#include <stddef.h>
#include <string.h>

struct cli_parser cli_parser_from(int argc, char **args) {
    struct cli_parser cli = { 0 };
    cli.args = args;
    cli.flags.hasheq = ht_cstr_hasheq;
    cli.options.hasheq = ht_cstr_hasheq;

    for (size_t i = 0; i < argc; ++i) {
        char *arg = args[i];
        size_t arg_len = strlen(arg);
        if (arg_len && arg[0] == '-') {
            if (arg_len > 1 && arg[1] == '-') {
                char *flag = arg + 2;
                *ht_put(&cli.flags, flag) = 1;
            }
        }
    }

    return cli;
}
