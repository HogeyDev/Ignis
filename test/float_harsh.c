#include <stdio.h>

int main() {
    float x = 4.0f;
    printf("%i\n", *(int *)&x); 

    return 0;
}
