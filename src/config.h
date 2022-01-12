#pragma once

#include "bool.h"

#define PROGRAM_NAME "main"
#define printVerbose if(getVerbose()) printf

// Logging
void setVerbose(bool value);
bool getVerbose(); // More verbose output, meant for debugging

// Stack
void setInitialStackSize(unsigned long value);
unsigned long getInitialStackSize(); // Starting size for stack
void setStackExpansionStepSize(unsigned long value);
unsigned long getStackExpansionStepSize(); // How much to allocate for the stack upon overflow
void setExpandableStack(bool value);
bool getExpandableStack(); // If the stack may expand

// Runtime/VM
void setPanic(bool value);
bool getPanic();