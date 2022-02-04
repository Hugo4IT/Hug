#include "stack.h"

Stack newStack(uint32_t size) {
    uint8_t *data = malloc(size);
    return (Stack){data};
}

void writeStack(Stack *stack, uint32_t position, uint8_t *data, uint32_t dataSize) {
    uint8_t *dataPointer = &data[0];
    uint8_t *stackPointer = &stack->data[position];
    while (dataSize--) *(stackPointer++) = *(dataPointer++);
}

void readStack(Stack *stack, uint32_t position, uint32_t dataSize, uint8_t *buffer) {
    uint8_t *bufferPointer = &buffer[0];
    uint8_t *stackPointer = &stack->data[position];
    while (dataSize--) *(bufferPointer++) = *(stackPointer++);
}

void destroyStack(Stack *stack) {
    free(stack->data);
    stack = NULL;
}