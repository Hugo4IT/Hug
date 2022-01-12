#include "lexer.h"
#include "config.h"
#include <stdlib.h>
#include <string.h>

bool startsWith(char *haystack, char *needle) {
    char ch; int i = 0;
    while((ch = needle[i]) != '\0' && haystack[i] != '\0')
        if (haystack[i++] != ch) return false;
    return true;
}

Operation newEmptyOperation() {
    return (Operation){Empty, NULL, 0, 0};
}

void pushOperation(AbstractSyntaxTree *tree, Operation symbol) {
    while (sizeof(Operation) * tree->operationCount >= tree->operationHolderSize) {
        tree->operationHolderSize += sizeof(Operation) * MEMORY_EXPANSION_STEP;
        tree->operations = realloc(tree->operations, tree->operationHolderSize);
    }
    tree->operations[tree->operationCount++] = symbol;
}

void pushCharacter(Operation *operation, char ch) {
    while (operation->dataLength >= operation->dataSize) {
        operation->dataSize += MEMORY_EXPANSION_STEP;
        operation->data = realloc(operation->data, operation->dataSize);
    }
    operation->data[operation->dataLength++] = ch;
}

AbstractSyntaxTree getAbstractSyntaxTree(char *program) {
    AbstractSyntaxTree tree = (AbstractSyntaxTree){0UL, 0UL, NULL};

    int i = 0; char ch = '\0';
    bool isEscaped = false;
    bool isFinished = false;
    char operatorBuffer[40];
    int operatorBufferIndex = 0;
    while (true) {
        Operation operation = newEmptyOperation();
        while (!isFinished) {
            ch = program[i++];
            if (ch == '\0') break;
            if (operation.operator == Empty) {
                switch (ch) {
                    case '@':
                        operation.operator = PushCallStack;
                        break;
                    case ' ':
                        if (startsWith(&operatorBuffer[0], "push")) {
                            operation.operator = PushToStack;
                        } else if (startsWith(&operatorBuffer[0], "print")) {
                            operation.operator = PrintStack;
                            isFinished = true;
                        } else if (startsWith(&operatorBuffer[0], "call")) {
                            operation.operator = PushCallStack;
                            operation.dataSize = sizeof(unsigned long);
                            operation.data = malloc(operation.dataSize);
                            memcpy(operation.data, (unsigned long*)&i, operation.dataSize);
                        } else {
                            fprintf(stderr, "[ERROR] Unrecognized operator: %s at %d\n", operatorBuffer, i);
                            operation.operator = Empty;
                            if (operation.dataSize > 0) {
                                free(operation.data);
                                operation.dataSize = 0;
                            }
                            isFinished = true;
                        }
                        // Clear buffer
                        for (int _ = 0; _ < operatorBufferIndex; _++)
                            operatorBuffer[--operatorBufferIndex] = 0;
                        break;
                    default:
                        if (operatorBufferIndex >= 39) {
                            fprintf(stderr, "[ERROR] Unrecognized operator: %s at %d\n", operatorBuffer, i);
                            operation.operator = Empty;
                            if (operation.dataSize > 0) {
                                free(operation.data);
                                operation.dataSize = 0;
                            }
                            isFinished = true;
                        } else {
                            operatorBuffer[operatorBufferIndex] = ch;
                        }
                        break;
                }
            } else {
                switch (operation.operator) {
                    case PushToStack:
                        if (operation.dataSize == 0) {
                            if (ch == '"') {
                                operation.data = realloc(operation.data, operation.dataSize);
                                operation.dataSize = MEMORY_EXPANSION_STEP;
                                operation.dataLength = 0;
                            }
                        } else {
                            if (!isEscaped) {
                                if (ch == '\\') isEscaped = true;
                                else {
                                    if (ch == '\"') {
                                        operation.dataSize = operation.dataLength;
                                        operation.data = realloc(operation.data, operation.dataSize);
                                        isFinished = true;
                                    } else if (ch == 'n') {
                                        pushCharacter(&operation, '\n');
                                    } else {
                                        pushCharacter(&operation, ch);
                                    }
                                }
                            } else {
                                if (ch == '"' || ch == '\'')
                                    pushCharacter(&operation, ch);
                                printVerbose("Escaped character: %c\n", ch);
                                isEscaped = false;
                            }
                        }
                        break;
                }
            }
        }

        if (operation.operator != Empty) {
            if (getVerbose()) {
                printf("Found operator %d", operation.operator);
                if (operation.dataLength > 0) printf(" with data: ");
                for (unsigned long i = 0; i < operation.dataLength; i++) {
                    printf("%c", operation.data[i]);
                }
                printf("\n");
            }
            pushOperation(&tree, operation);
        }
        isFinished = false;
        if (ch == '\0') break;
    }

    // Shrink back down
    tree.operationHolderSize = sizeof(Operation) * tree.operationCount;
    tree.operations = realloc(tree.operations, tree.operationHolderSize);

    return tree;
}

void destroyAbstractSyntaxTree(AbstractSyntaxTree *tree) {
    for (unsigned long i = 0; i < tree->operationCount; i++)
        if (tree->operations[i].dataSize)
            free(tree->operations[i].data);
    free(tree->operations);
}