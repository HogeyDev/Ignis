int (*(*A(int))(int))(float x, float y, int race);

int main() {
    int (*(*(*a)(int))(int))(float, float, int) = A;
    a(4);
}
