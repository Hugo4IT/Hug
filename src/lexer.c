#include "lexer.h"

const char DELIMITERS[] = {'{', '}', '(', ')'};
const char DELIMITER_COUNT = 6;

bool isDelimiter(char ch) {
    for (int i = 0; i < DELIMITER_COUNT; i++)
        if (ch == DELIMITERS[i]) return true;
    return false;
}

bool isWhitespcace(char ch) {
    return ch == ' ' || ch == '\n';
}

typedef struct MappedAddress {
    char *name;
    unsigned long location;
} MappedAddress;

typedef struct AddressMap {
    MappedAddress *maps;
    unsigned long mapCount;
} AddressMap;

typedef struct ScriptReader {
    unsigned long index;
    char *script;
    unsigned long bufferLength;
    unsigned long bufferSize;
    char *buffer;
} ScriptReader;

AddressMap newAddressMap() {
    return (AddressMap){NULL, 0};
}

void mapAddress(AddressMap *mapping, char* name, unsigned long location) {
    mapping->maps = realloc(mapping->maps, ++mapping->mapCount * sizeof(MappedAddress));
    mapping->maps[mapping->mapCount-1] = (MappedAddress){name, location};
}

unsigned long getAddress(AddressMap *mapping, char *name) {
    for (unsigned long i = 0; i < mapping->mapCount; i++)
        if (strcmp(mapping->maps[i].name, name) == 0)
            return mapping->maps[i].location;
    free(name);

    fprintf(stderr, "[ERROR]: Could not find symbol %s in mapped addressed. Check if the name matches the target function/variable.\n", name);
    return 0;
}

void destroyAddressMap(AddressMap *mapping) {
    for (unsigned long i = 0; i < mapping->mapCount; i++)
        free(mapping->maps[i].name);
    free(mapping->maps);
}

Operation newEmptyOperation() {
    return (Operation){Empty, NULL};
}

void pushOperation(AbstractSyntaxTree *tree, Operation symbol) {
    while (sizeof(Operation) * tree->operationCount >= tree->operationHolderSize) {
        tree->operationHolderSize += sizeof(Operation) * MEMORY_EXPANSION_STEP;
        tree->operations = realloc(tree->operations, tree->operationHolderSize);
    }
    tree->operations[tree->operationCount++] = symbol;
}

ScriptReader newReader(char *program) {
    return (ScriptReader){0, program, 0, 1024, malloc(1024)};
}

void pushReaderBuffer(ScriptReader *reader, char ch) {
    if (reader->bufferSize <= reader->bufferLength) {
        reader->bufferSize += 1024;
        reader->buffer = realloc(reader->buffer, reader->bufferSize);
    }
    reader->buffer[reader->bufferLength++] = ch;
}

void clearReaderBuffer(ScriptReader *reader) {
    for (unsigned long i = 0; i < reader->bufferLength; i++)
        reader->buffer[i] = '\0';
    reader->bufferLength = 0;
}

char getNextCharacter(ScriptReader *reader) {
    return reader->script[++reader->index];
}

char *getNextSymbol(ScriptReader *reader) {
    char ch = reader->script[reader->index];
    while (isWhitespcace(ch)) ch = getNextCharacter(reader);
    while (!isDelimiter(ch) && !isWhitespcace(ch) && ch != '\0' && ch != EOF) {
        if (ch == '"') { // Strings
            ch = getNextCharacter(reader);
            while (ch != '"') {
                pushReaderBuffer(reader, ch);
                ch = getNextCharacter(reader);
            }
            break;
        } else {
            pushReaderBuffer(reader, ch);
            ch = getNextCharacter(reader);

            // Move back one character, so the main run() loop will find it in "ch"
            if (isDelimiter(ch)) reader->index--;
        }
    }

    // Copy readable part of reader->buffer and append with nul terminator
    char *symbolBuffer = malloc(reader->bufferLength + 1);
    for (unsigned long i = 0; i < reader->bufferLength; i++)
        symbolBuffer[i] = reader->buffer[i];
    symbolBuffer[reader->bufferLength] = '\0';

    clearReaderBuffer(reader);

    return symbolBuffer;
}

void destroyReader(ScriptReader *reader) {
    free(reader->script);
    free(reader->buffer);
}

AbstractSyntaxTree getAbstractSyntaxTree(char *program) {
    AbstractSyntaxTree tree = (AbstractSyntaxTree){0UL, 0UL, NULL};
    ScriptReader reader = newReader(program);
    AddressMap mapping = newAddressMap();

    bool exitLoop = false;
    char ch = '\n';
    while (!exitLoop) {
        switch (ch) {
            case '\0':
            case EOF:
                exitLoop = true;
                break;
            default:
                {} // Clangd be annoying :/
                char *symbol = getNextSymbol(&reader);
                Operation operation = newEmptyOperation();

                if (strcmp(symbol, "function") == 0) {
                    mapAddress(&mapping, getNextSymbol(&reader), tree.operationCount);
                    operation.operator = Function;
                } else if (strcmp(symbol, "push") == 0) {
                    operation.operator = PushToStack;
                    operation.data = getNextSymbol(&reader);
                } else if (strcmp(symbol, "print") == 0) {
                    operation.operator = PrintStack;
                } else if (strcmp(symbol, "call") == 0) {
                    char *nextSymbol = getNextSymbol(&reader);
                    operation.operator = PushCallStack;
                    operation.data = (char*)getAddress(&mapping, nextSymbol);
                    free(nextSymbol);
                } else if (ch == '}') {
                    operation.operator = PopCallStack;
                }
                
                free(symbol);
                if (operation.operator != Empty)
                    pushOperation(&tree, operation);
                break;
        }
        
        ch = getNextCharacter(&reader);
    }

    destroyReader(&reader);
    destroyAddressMap(&mapping);

    // Shrink back down
    tree.operationHolderSize = sizeof(Operation) * tree.operationCount;
    tree.operations = realloc(tree.operations, tree.operationHolderSize);

    return tree;
}

void destroyAbstractSyntaxTree(AbstractSyntaxTree *tree) {
    for (unsigned long i = 0; i < tree->operationCount; i++)
        if (tree->operations[i].operator == PushToStack)
            free(tree->operations[i].data);
    free(tree->operations);
}