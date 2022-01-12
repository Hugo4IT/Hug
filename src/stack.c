#include "stack.h"
#include "config.h"

Stack newStack() {
    char *data = malloc(getInitialStackSize());
    for (unsigned long i = 0; i < getInitialStackSize(); i++) data[i] = '\0';
    return (Stack) {getInitialStackSize(), 0, data};
}

void pushToStack(Stack *stack, char *data, unsigned long dataSize) {
    for (unsigned long i = 0; i < dataSize; i++) {
        while (stack->stackPointer >= stack->dataSize) {
            stack->dataSize += getStackExpansionStepSize();
            stack->data = realloc(stack->data, stack->dataSize);
        }
        stack->data[stack->stackPointer++] = data[i];
    }
}

void popStackWithoutBuffer(Stack *stack, unsigned long dataSize) {
    for (unsigned long i = 0; i < dataSize; i++)
        stack->data[--stack->stackPointer] = 0;
}

void popStackToBuffer(Stack *stack, char *buffer, unsigned long dataSize) {
    stack->stackPointer -= dataSize;
    for (unsigned long i = 0; i < dataSize; i++)
        buffer[i] = stack->data[stack->stackPointer + i];
}

char *popStack(Stack *stack, unsigned long dataSize) {
    char *data = malloc(dataSize);
    popStackToBuffer(stack, data, dataSize);
    return data;
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