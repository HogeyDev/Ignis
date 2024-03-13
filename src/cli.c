#include "cli.h"
#include <stdlib.h>
#include <string.h>

CLI_Parser *parse_cli(CLI_Parser *cli_parser, char **argv) {
  const char *program_name = "example/hello_world";

  cli_parser->input_file = malloc(strlen(program_name) + strlen(".is") + 1);
  strcpy(cli_parser->input_file, program_name);
  strcat(cli_parser->input_file, ".is");
  cli_parser->output_file = malloc(strlen(program_name) + 1);
  strcpy(cli_parser->output_file, program_name);

  return cli_parser;
}
