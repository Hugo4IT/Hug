#include "config.h"

static bool isVerbose = false;

void setVerbose(bool value) {
    isVerbose = value;
}

bool getVerbose() {
    return isVerbose;
}

static unsigned long initialStackSize = 128;

void setInitialStackSize(unsigned long value) {
    initialStackSize = value;
}

unsigned long getInitialStackSize() {
    return initialStackSize;
}

static unsigned long stackExpansionStepSize = 128;

void setStackExpansionStepSize(unsigned long value) {
    stackExpansionStepSize = value;
}

unsigned long getStackExpansionStepSize() {
    return stackExpansionStepSize;
}

static bool shouldPanic = false;

void setPanic(bool value) {
    shouldPanic = value;
}

bool getPanic() {
    return shouldPanic;
}