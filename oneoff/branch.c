#include <limits.h>

int wb(unsigned int x) {
    while ((x | 1) > 0) {
        return 0;
    }
}

int main() {
    return wb(4);
}
