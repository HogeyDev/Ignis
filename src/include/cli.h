#ifndef CLI_H
#define CLI_H

typedef struct {
  char *argv;

  char *input_file;
  char *output_file;
} CLI_Parser;

CLI_Parser *parse_cli(CLI_Parser *cli_parser, char **argv);

#endif // !CLI_H
