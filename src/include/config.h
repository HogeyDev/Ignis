#ifndef CONFIG_H
#define CONFIG_H

#include "cli.h"

typedef struct {
    char **items;
    int count;
    int capacity;
} ImportedFiles;

typedef struct {
    char *main_file;
    char *root_path;
    char *std_path;
    ImportedFiles imported_files;
} ProgramConfig;

ProgramConfig get_config(char *main_file, CliParser *clip);
void config_free(ProgramConfig *config);

#endif
