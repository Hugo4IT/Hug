#pragma once

#include "bool.h"
#include "config.h"

#include <stdio.h>
#include <stdlib.h>

#define MEMORY_EXPANSION_STEP 40

enum Operators {
    Empty,
    PushToStack,
    PrintStack
};

typedef struct Operation {
    char operator;
    char *data;
    unsigned long dataSize;
    unsigned long dataLength;
} Operation;

typedef struct AbstractSyntaxTree {
    unsigned long operationHolderSize;
    unsigned long operationCount;
    Operation *operations;
} AbstractSyntaxTree;

AbstractSyntaxTree getAbstractSyntaxTree(char *program);
void destroyAbstractSyntaxTree(AbstractSyntaxTree *tree);