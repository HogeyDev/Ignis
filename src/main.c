#include <stdio.h>
#include "cli.h"
#define HT_IMPLEMENTATION
#include "ht.h"

int main(int argc, char **argv) {
    struct cli_parser clip = cli_parser_from(argc, argv);
    ht_foreach(value, &clip.flags) {
        printf("%d\n", *value);
    }
    ht_free(&clip.flags);
    ht_free(&clip.options);

    return 0;
}
