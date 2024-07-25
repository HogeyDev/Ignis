#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define IGNIS_PATH "../target/debug/ignis"
#define FILE_ENDING ".is"

typedef struct {
    const char *binary_path;
    const char *source_path;
    const char *output;
    int return_code;
} Test;

typedef struct {
    Test *tests;
    unsigned int size;
    unsigned int capacity;
} Tests;

Tests initTests() {
    Tests tests = { 0 };

    tests.size = 0;
    tests.capacity = 8;
    tests.tests = malloc(tests.capacity * sizeof(Test));

    return tests;
}

void addTest(Tests *tests, const char *binary_path, const char *output, int return_code) {
    if (tests->size >= tests->capacity - 1) {
        tests->capacity *= 2;
        tests->tests = realloc(tests->tests, sizeof(Test) * tests->capacity);
    }
    Test test = { 0 };
    test.output = output;
    test.binary_path = binary_path;
    char *source_path = (char *)malloc((strlen(binary_path) + strlen(FILE_ENDING)) * sizeof(char));
    strcpy(source_path, binary_path);
    strcat(source_path, FILE_ENDING);
    test.source_path = source_path;
    test.return_code = return_code;
    tests->tests[tests->size] = test;
    tests->size++;
}

void runTests(Tests *tests) {
    for (unsigned int i = 0; i < tests->size; i++) {
        Test test = tests->tests[i];
        printf("[%d/%d] Testing `%s`\n", i+1, tests->size, test.binary_path);

        {
            char *compile_command = (char *)malloc((strlen("RUST_BACKTRACE=1 ") + strlen(IGNIS_PATH) + strlen(" -o ") + strlen(test.binary_path) + strlen(" ") + strlen(test.source_path) + strlen(" --debug-asm") + strlen(" --debug-ast")) * sizeof(char));
            sprintf(compile_command, "RUST_BACKTRACE=1 %s -o %s %s --debug-asm --debug-asr", IGNIS_PATH, test.binary_path, test.source_path);
            printf("COMPILING: %s\n", compile_command);
            unsigned int code = WEXITSTATUS(system(compile_command));
            if (code != 0) {
                printf("Test `%s` failed to compile\n", test.binary_path);
                exit(1);
            }
        }

        {
            char *run_command = (char *)malloc((strlen("./") + strlen(test.binary_path)) * sizeof(char));
            sprintf(run_command, "./%s", test.binary_path);
            printf("RUNNING: %s\n", run_command);
            unsigned int code = WEXITSTATUS(system(run_command));
            if (code != test.return_code) {
                printf("Test `%s` failed\n\t`%s` expected exit code `%d`, but exited `%d` instead\n", test.binary_path, test.binary_path, test.return_code, code);
                exit(1);
            }
        }
    }
    printf("\nAll tests passed successfully!\n");
}

int main() {
    Tests tests = initTests();

    addTest(&tests, "fibonacci", "", 13);
    addTest(&tests, "person", "", 17);
    addTest(&tests, "return", "", 19);

    runTests(&tests);
    
    return 0;
}
