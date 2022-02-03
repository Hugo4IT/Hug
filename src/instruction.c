#include "instruction.h"

Instruction newInstruction(uint8_t opCode, uint8_t *data) {
	return (Instruction){opCode, data};
}

void destroyInstruction(Instruction *instruction) {
	if (instruction->data) free(instruction->data);
	instruction = NULL;
}