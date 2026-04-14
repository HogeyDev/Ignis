#include "io.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

SourceFile io_read_file(char *path) {
    FILE *fp = fopen(path, "rb");

    fseek(fp, 0L, SEEK_END);
    long file_size = ftell(fp);
    rewind(fp);

    char *contents = (char *)malloc((file_size + 1) * sizeof(char));
    size_t bytes_read = fread(contents, sizeof(char), file_size, fp);
    if (bytes_read != file_size) {
        fprintf(stderr, "could not read file '%s'\n", path);
        exit(1);
    }
    contents[bytes_read] = '\0';

    return (SourceFile){
        .path = path,
        .contents = contents,
    };
}
