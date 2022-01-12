#include "vm.h"
#include "bool.h"
#include "lexer.h"
#include "fileio.h"
#include "config.h"

#include <stdlib.h>

void printUsage() {
    printf("Usage:\n");
    printf("  %s [options] <file>\n", PROGRAM_NAME);
    printf("Options:\n");
    printf("  -v,--verbose                       Verbose output, mostly useful for debugging\n");
    printf("  -h,--help                          Print this helpfully helpful helping help message\n");
    printf("  -s,--initial-stack-size <size>     Pre-allocate <size> bytes for the stack \n");
    printf("  -S,--stack-expansion-step <size>   When the stack limit is reached, allocate <size> more bytes\n");
}

bool startsWith(char *haystack, char *needle) {
    char ch; int i = 0;
    while((ch = needle[i]) != '\0' && haystack[i] != '\0')
        if (haystack[i++] != ch) return false;
    return true;
}

void writeString(char *source, char *destination) {
    char ch; int i = 0;
    while((ch = source[i]) != '\0' && destination[i] != '\0')
        destination[i++] = ch;
}

int main(int argc, const char** argv) {
    const char *inputFile;
    bool hasReceivedInput = false;

    if (argc < 2) {
        fprintf(stderr, "[ERROR] Not enough arguments given.\n");
        printUsage();
        return EXIT_FAILURE;
    } else {
        // Used for an argument that takes input (e.g. --target <target>)
        //
        // 0 = None
        // 1 = --with-stack-size
        // 2 = --stack-expansion-step
        char previousArg = 0;
        for (int i = 1; i < argc; i++) {
            char *currentArg = (char*)argv[i];
            if (previousArg == 0) {
                if (startsWith(currentArg, "--verbose") || startsWith(currentArg, "-v")) {
                    setVerbose(true);
                    previousArg = 0;
                } else if (startsWith(currentArg, "--help") || startsWith(currentArg, "-h")) {
                    printUsage();
                    return EXIT_SUCCESS;
                } else if (startsWith(currentArg, "--initial-stack-size") || startsWith(currentArg, "-s")) {
                    previousArg = 1;
                } else if (startsWith(currentArg, "--stack-expansion-step") || startsWith(currentArg, "-S")) {
                    previousArg = 2;
                } else {
                    inputFile = currentArg;
                    hasReceivedInput = true;
                }
            } else {
                switch (previousArg) {
                    case 1:
                        setInitialStackSize((unsigned long) atoi(currentArg));
                        break;
                    case 2:
                        setStackExpansionStepSize((unsigned long) atoi(currentArg));
                        if (getStackExpansionStepSize() == 0) {
                            fprintf(stderr, "[ERROR] --stack-expansion-step may not be 0");
                            return EXIT_FAILURE;
                        }
                        break;
                    default:
                        fprintf(stderr, "[ERROR] Something went horribly wrong... :/\n");
                        return EXIT_FAILURE;
                }
            }
        }
    }

    if (inputFile == NULL || !hasReceivedInput) {
        fprintf(stderr, "[ERROR] No input file given\n");
        printUsage();
        return EXIT_FAILURE;
    }

    printVerbose("Verbose output enabled.\n");

    char *fileContents = getFileContents(inputFile);
    if (fileContents == NULL) {
        fprintf(stderr, "[ERROR] Couldn't read file %s", inputFile);
        return EXIT_FAILURE;
    }

    printVerbose("\n---File content---\n\n%s\n\n------------------\n\n", fileContents);

    AbstractSyntaxTree tree = getAbstractSyntaxTree(fileContents);

    printVerbose("\n------script------\n\n");

    int result = run(tree);
    if (result != Success) {
        fprintf(stderr, "[ERROR]: %s\n", getRunError(result));
    }

    printVerbose("\n------------------\n\n");


    destroyAbstractSyntaxTree(&tree);
    free(fileContents);
    return EXIT_SUCCESS;
}
