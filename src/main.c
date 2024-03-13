#include "cli.h"
#include "compile.h"
#include "io.h"
#include <stdio.h>

int main(int argc, char **argv) {
  CLI_Parser cli_parser = {0};
  (void)parse_cli(&cli_parser, argv);
  char *file_contents = read_file(cli_parser.input_file);
  char *compiled = compile(file_contents);
  write_file(cli_parser.output_file, compiled);
  return 0;
}
