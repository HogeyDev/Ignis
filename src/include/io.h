#ifndef IO_H
#define IO_H

#define FILE_IO_BUFFER_SIZE 1024

char *read_file(char *file_path);
int write_file(char *file_path, char *file_contents);

#endif // !IO_H
