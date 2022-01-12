#include "stack.h"
#include "config.h"
#include <stdlib.h>

Stack newStack() {
    return (Stack) {getInitialStackSize(), 0, malloc(getInitialStackSize())};
}

void pushToStack(Stack *stack, char *data, unsigned long dataLength) {
    for (unsigned long i = 0; i < dataLength; i++) {
        while (stack->stackPointer >= stack->dataSize) {
            if (getExpandableStack()) {
                stack->dataSize += getStackExpansionStepSize();
                stack->data = realloc(stack->data, stack->dataSize);
            } else
                stack->stackPointer = 0;
        }
        stack->data[stack->stackPointer++] = data[i];
    }
}

char *getStackSlice(Stack *stack, unsigned long from, unsigned long to) {
    char *slice = malloc(to - from);
    for (unsigned long i = 0; i < to - from; i++)
        slice[i] = stack->data[from + i];
    return slice;
}

void destroyStack(Stack *stack) {
    free(stack->data);
}