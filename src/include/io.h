#ifndef IO_H
#define IO_H

typedef struct {
    char *path;
    char *contents;
} SourceFile;

SourceFile io_read_file(char *path);

#endif
