#include "io.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *read_file(char *file_path) {
  FILE *file_pointer = fopen(file_path, "r");

  if (file_pointer == NULL) {
    fprintf(stderr, "Could not open file: %s\n", file_path);
    exit(1);
  }

  char *file_contents = malloc(0);

  char buffer[FILE_IO_BUFFER_SIZE];
  while (fgets(buffer, sizeof(buffer), file_pointer) != NULL) {
    file_contents =
        realloc(file_contents, strlen(file_contents) + FILE_IO_BUFFER_SIZE + 1);
    strcat(file_contents, buffer);
  }

  file_contents = realloc(file_contents, strlen(file_contents) + 1);

  fclose(file_pointer);

  return file_contents;
}

int write_file(char *file_path, char *file_contents) {
  FILE *file_pointer = fopen(file_path, "w");

  if (file_pointer == NULL) {
    fprintf(stderr, "Could not open file: %s", file_path);
    exit(1);
  }

  fprintf(file_pointer, "%s\n", file_contents);

  fclose(file_pointer);

  return 0;
}
