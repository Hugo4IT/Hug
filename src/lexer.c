#include "lexer.h"
#include "config.h"
#include <stdlib.h>

typedef struct MappedAddress {
    char *name;
    unsigned long location;
} MappedAddress;

typedef struct AddressMap {
    MappedAddress *maps;
    unsigned long mapCount;
} AddressMap;

AddressMap newAdressMap() {
    return (AddressMap){NULL, 0};
}

void mapAddress(AddressMap *mapping, char name[39], unsigned long location) {
    mapping->maps = realloc(mapping->maps, ++mapping->mapCount * sizeof(MappedAddress));
    mapping->maps[mapping->mapCount-1] = (MappedAddress){name, location};
}

unsigned long getAddress(AddressMap *mapping, char name[40]) {
    for (unsigned long i = 0; i < mapping->mapCount; i++)
        if (stringEquals(mapping->maps[i].name, name))
            return mapping->maps[i].location;

    fprintf(stderr, "[ERROR]: Could not find symbol %s in mapped addressed. Check if the name matches the target function/variable.\n", name);
    return 0;
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
    AddressMap mapping = newAdressMap();

    int i = 0; char ch = '\0';
    bool isEscaped = false;
    bool isFinished = false;
    bool awaitWord = false;
    char operatorBuffer[40];
    for (int i = 0; i < 40; i++) operatorBuffer[i] = '\0';
    int operatorBufferIndex = 0;
    while (true) {
        Operation operation = newEmptyOperation();
        while (!isFinished) {
            ch = program[i++];
            if (operation.operator == Empty || awaitWord) {
                switch (ch) {
                    case '@':
                        if (operation.operator == Empty)
                            operation.operator = Function;
                        awaitWord = true;
                        break;
                    case '}':
                        operation.operator = PopCallStack;
                        isFinished = true;
                        break;
                    case '{':
                    case ' ':
                    case '\n':
                    case EOF:
                    case '\0':
                        awaitWord = false;

                        if (operation.operator == Function) {
                            mapAddress(&mapping, &operatorBuffer[1], tree.operationCount); // Start from index 1 to remove "@"
                            isFinished = true;
                        } else if (operation.operator == PushCallStack) {
                            operation.data = (char*)getAddress(&mapping, &operatorBuffer[0]);
                            isFinished = true;
                        } else if (startsWith(&operatorBuffer[0], "push")) {
                            operation.operator = PushToStack;
                        } else if (startsWith(&operatorBuffer[0], "print")) {
                            operation.operator = PrintStack;
                            isFinished = true;
                        } else if (startsWith(&operatorBuffer[0], "call")) {
                            operation.operator = PushCallStack;
                            awaitWord = true;
                        } else if (operatorBuffer[0] != '\0') {
                            fprintf(stderr, "[ERROR] Unrecognized operator: %s at %d/%d\n", operatorBuffer, i, __LINE__);
                            operation.operator = Empty;
                            if (operation.dataSize > 0) {
                                free(operation.data);
                                operation.dataSize = 0;
                            }
                        }
                        // Clear buffer
                        for (int i = 0; i < operatorBufferIndex; i++)
                            operatorBuffer[i] = '\0';
                        operatorBufferIndex = 0;
                        break;
                    default:
                        if (operatorBufferIndex >= 39) {
                            fprintf(stderr, "[ERROR] Unrecognized operator: %s at %d/%d\n", operatorBuffer, i, __LINE__);
                            operation.operator = Empty;
                            if (operation.dataSize > 0) {
                                free(operation.data);
                                operation.dataSize = 0;
                            }
                            isFinished = true;
                        } else operatorBuffer[operatorBufferIndex++] = ch;
                        break;
                }
            } else {
                switch (operation.operator) {
                    case PushToStack:
                        if (!isEscaped) {
                            if (ch == '\\') isEscaped = true;
                            else {
                                if (ch == '\"') {
                                    if (operation.dataSize != 0) {
                                        operation.dataSize = operation.dataLength + 1;
                                        operation.data = realloc(operation.data, operation.dataSize);
                                        operation.data[operation.dataLength] = '\0';
                                        isFinished = true;
                                    }
                                } else {
                                    pushCharacter(&operation, ch);
                                }
                            }
                        } else {
                            if (ch == '"' || ch == '\'')
                                pushCharacter(&operation, ch);
                            else if (ch == 'n') {
                                pushCharacter(&operation, '\n');
                            } 
                            printVerbose("Escaped character: %c\n", ch);
                            isEscaped = false;
                        }
                        break;
                }
            }

            if (ch == '\0') break;
        }

        if (operation.operator != Empty)
            pushOperation(&tree, operation);
        isFinished = false;
        if (ch == '\0' || ch == EOF) break;
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