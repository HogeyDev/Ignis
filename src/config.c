#include "config.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <limits.h>

#include "cli.h"

ProgramConfig get_config(char *main_file, CliParser *clip) {
    ProgramConfig config = { 0 };
    config.main_file = main_file;

    char cwd[PATH_MAX];
    if (!getcwd(cwd, sizeof(cwd))) {
        fprintf(stderr, "could not get current working directory");
        exit(1);
    }
    size_t len = strlen(cwd);
    if (cwd[len-1] == '/') {
        config.root_path = strdup(cwd);
    } else {
        config.root_path = malloc((len + 2) * sizeof(char));
        strcpy(config.root_path, cwd);
        config.root_path[len] = '/';
    }

    config.std_path = cli_option_fallback(clip, "stdlib", "/home/iris/Programming/Ignis/std/"); // TODO: replace this with something a little less hard-coded lol
    config.std_path = realpath(config.std_path, NULL);
    if (config.std_path == NULL) {
        fprintf(stderr, "could not resolve stdlib path\n");
        exit(1);
    }

    return config;
}

void config_free(ProgramConfig *config) {
    free(config->root_path);
    free(config->std_path);
}
