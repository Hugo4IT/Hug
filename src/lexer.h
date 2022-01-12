#pragma once

#include "bool.h"
#include "config.h"

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include "strings.h"

#define MEMORY_EXPANSION_STEP 40

enum Operators {
    Empty,
    PushToStack,
    PrintStack,
    PushCallStack,
    PopCallStack,
    Function
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