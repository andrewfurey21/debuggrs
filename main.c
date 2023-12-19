#include "stdio.h"

void fibonacci(int max) {
    int a = 0;
    int b = 1;

    while (a <= max) {
        printf("%d ", a);
        int c = a + b;
        a = b;
        b = c;
    }

    printf("\n");
}

int main() {
   int max = 0;
   printf("Fibonacci up to ");
   scanf("%d", &max);

   fibonacci(max);
}
