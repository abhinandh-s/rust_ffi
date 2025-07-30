#include <stdio.h>

// Simple hello function
void hello_from_c(void) {
    printf("Hello from C!\n");
}

// Function that takes a parameter
void greet_person(const char* name) {
    printf("Hello, %s! Greetings from C.\n", name);
}

// Function that returns a value
int add_numbers(int a, int b) {
    return a + b;
}
