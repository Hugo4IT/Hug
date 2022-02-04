#pragma once

#include <stdlib.h>
#include <stdint.h>

#define INSTRUCTION_NOOP 0x01
#define INSTRUCTION_STACK_WRITE 0x02
#define INSTRUCTION_STACK_READ 0x03

typedef struct Instruction {
	uint8_t opCode;
	uint8_t *data;
} Instruction;

Instruction newInstruction(uint8_t opCode, uint8_t *data);
void destroyInstruction(Instruction *instruction);