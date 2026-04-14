#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "cli.h"
#include "config.h"
#include "da.h"
#include "io.h"
#include "tokenizer.h"

int main(int argc, char **argv) {
    CliParser clip = cli_parser_from(argc, argv);

    do_easter_eggs(&clip);

    if (clip.arguments.count != 1) {
        fprintf(stderr, "\e[0;31merror\e[0m: %s\n\tusage: %s path/to/main.is -o path/to/output\n",
                clip.arguments.count > 1 ? "too many main files listed" : "no main file specified",
                clip.args[0]);
        exit(1);
    }

    char *input_file_path = clip.arguments.items[0];
    SourceFile input_file = io_read_file(input_file_path);

    ProgramConfig config = get_config(input_file_path, &clip);

    Tokens tokens = get_tokens(&input_file);
    for (size_t i = 0; i < tokens.count; i++) {
        Token token = tokens.items[i];
        printf("%zu: %s\n", i, format_token(token));
    }

    config_free(&config);
    free(input_file.contents);
    da_free(&clip.flags);
    da_free(&clip.options);
    da_free(&clip.arguments);
    return 0;
}
