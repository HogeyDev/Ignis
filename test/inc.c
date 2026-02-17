#include <stdio.h>
#include "inc.h"

int main(void) {
    x = 10;
    printf("Global: %d\n", x);

    #include "inc.h"
    printf("Local: %d\n", x);

    return 0;
}
