#include "script.h"

Script newScript() {
	return (Script){
		0,
		0,
		128 * sizeof(Instruction),
		malloc(128 * sizeof(Instruction))
	};
}

void pushInstructionToScript(Script *script, Instruction instruction) {
	if (script->instructionCount * sizeof(Instruction) >= script->instructionsSize) {
		script->instructionsSize = (script->instructionCount + 128) * sizeof(Instruction);
		script->instructions = realloc(script->instructions, script->instructionsSize);
	}

	script->instructions[script->instructionCount++] = instruction;
}

void shrinkScript(Script *script) {
	script->instructionsSize = script->instructionCount * sizeof(Instruction);
	script->instructions = realloc(script->instructions, script->instructionsSize);
}

void destroyScript(Script *script) {
	free(script->instructions);
	script = NULL;
}