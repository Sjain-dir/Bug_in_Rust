#include <stdio.h>

void test1(int* i) {
    printf("in test 1 :::\n");
    printf("i pointer value is : %p\n",i);
    printf("i -> value in that address is : %d\n",*i);
}

void test2(char* string1, int *i) {
    printf("in test 2 :::\n");
    printf("i pointer value is : %p\n",i);
    printf("i value in that address is : %d\n",*i);
    printf("string1 value is : %s\n",string1);
}