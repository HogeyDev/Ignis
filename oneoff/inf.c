#include <math.h>
#include <stdio.h>
int main(void) {
    float x = INFINITY;
    printf("0x%x\n", *(int *)&x);
    return 0;
}
