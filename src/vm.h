#pragma once

#include "stack.h"
#include "lexer.h"
#include "config.h"

enum RunError {
    Success,
    RuntimeError
};

const char *getRunError(int result);
int run(AbstractSyntaxTree tree);
