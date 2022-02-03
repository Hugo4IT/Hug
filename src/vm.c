#include "vm.h"

// Compiled MAKr instructions

static void __i_noop(Instruction *instruction);
static void __i_stack_write(Instruction *instruction);
static void __i_stack_read(Instruction *instruction);

static void(*instructions[])(Instruction*) = {
    &__i_noop,
    &__i_stack_write,
    &__i_stack_read,
};

int runScript(Script *script) {
    
    return EXIT_SUCCESS;
}

void __i_noop(Instruction *instruction) {}

void __i_stack_write(Instruction *instruction) {

}

void __i_stack_read(Instruction *instruction) {
    
}