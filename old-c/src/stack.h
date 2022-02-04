#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef struct Stack {
    uint8_t *data;
} Stack;

Stack newStack(uint32_t size);
void writeStack(Stack *stack, uint32_t position, uint8_t *data, uint32_t dataSize);
void readStack(Stack *stack, uint32_t position, uint32_t dataSize, uint8_t *buffer);
void destroyStack(Stack *stack);