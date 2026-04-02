#include <stdio.h>
int func1(int x) {
    return 2 * x;
}
int func2(int x) {
    return x - 2;
}

int (*func(int selector))(int) {
    if (selector == 1) return func1;
    if (selector == 2) return func2;
}

int main(void) {

    return 0;
}
