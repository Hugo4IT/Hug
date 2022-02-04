#include "vm.h"
#include "lexer.h"
#include "fileio.h"
#include "script.h"
#include "strings.h"

#include <stdio.h>
#include <stdlib.h>

void printUsage() {
    printf("Usage:\n");
    printf("  makr <file>\n");
}

void writeString(char *source, char *destination) {
    char ch; int i = 0;
    while((ch = source[i]) != '\0' && destination[i] != '\0')
        destination[i++] = ch;
}

int main(int argc, const char** argv) {
    if (argc < 2) {
        fprintf(stderr, "[ERROR] Not enough arguments given.\n");
        printUsage();
        return EXIT_FAILURE;
    }

    char *fileContents = getFileContents(argv[1]);
    if (fileContents == NULL) {
        fprintf(stderr, "[ERROR] Couldn't read file %s\n", argv[1]);
        printUsage();
        return EXIT_FAILURE;
    }

    Script script = compileScript(fileContents);
    free(fileContents);

    int result = runScript(&script);
    if (result != EXIT_SUCCESS) {
        fprintf(stderr, "[ERROR]: %d\n", result);
    }

    destroyScript(&script);
    return EXIT_SUCCESS;
}
