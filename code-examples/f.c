#include <stdio.h>

int* f() {
    int a = 3;
    return &a;
}

int main() {
    printf("a: %i\n", *f());
}