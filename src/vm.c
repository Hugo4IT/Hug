#include "vm.h"
#include "stack.h"

void printStringBySize(char *string, unsigned long size) {
    for (unsigned long i = 0; i < size; i++)
        printf("%c", string[i]);
    printf("\n");
}

const char *getRunError(int result) {
    switch (result) {
        case Success:
            return "Nothing went wrong lol";
        case RuntimeError:
            return "A runtime error occurred! This means there might be a mistake in your program";
        default:
            return "Imagine programming so poorly, the interpreter breaks... Couldn't be me...";
    }
}

int run(AbstractSyntaxTree tree) {
    Stack stack = newStack();
    unsigned long executionPoint = 0;
    while (executionPoint < tree.operationCount) {
        bool moveToNextLine = true;

        Operation operation = tree.operations[executionPoint];
        switch (operation.operator) {
            case PushToStack:
                pushToStack(&stack, operation.data, operation.dataLength);
                break;
            case PrintStack:
                printStringBySize(stack.data, stack.dataSize);
                break;
            default: break;
        }

        if (getPanic()) {
            destroyStack(&stack);
            return RuntimeError;
        }
        if (moveToNextLine) executionPoint++;
    }
    
    destroyStack(&stack);
    return Success;
}