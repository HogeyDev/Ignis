#include <stdio.h>

int main(void) {
    const char *str = " World!";
    printf("%p\n", str);
    printf("|%c|\n", *(str+12));

    return 0;
}
