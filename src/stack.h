#pragma once

#include "config.h"

#include <stdio.h>
#include <stdlib.h>

typedef struct Stack {
    unsigned long dataSize;
    unsigned long stackPointer; // Current position in the stack
    char *data;
} Stack;

Stack newStack();
void pushToStack(Stack *stack, char *data, unsigned long dataSize);
void popStackWithoutBuffer(Stack *stack, unsigned long dataSize);
void popStackToBuffer(Stack *stack, char *buffer, unsigned long dataSize);
char *popStack(Stack *stack, unsigned long dataSize);
char *getStackSlice(Stack *stack, unsigned long from, unsigned long to);
void destroyStack(Stack *stack);