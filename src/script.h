#pragma once

#include <stdlib.h>
#include <stdint.h>

#include "instruction.h"

typedef struct Script {
	uint32_t stackSize;
	uint32_t instructionCount;
	uint32_t instructionsSize;
	Instruction *instructions;
} Script;

Script newScript();
void pushInstructionToScript(Script *script, Instruction instruction);
void shrinkScript(Script *script);
void destroyScript(Script *script);