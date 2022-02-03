#include "lexer.h"

bool isCharacter(char ch) {
    return ch >= '!' && ch <= '~';
}

bool isValidSymbolName(char ch) {
    return (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z');
}

void parseSymbol(Script *script, char **program) {
    uint32_t startOffset = 0;
    while (!isCharacter(**program)) (*program)++;
    while (**program) {
        char ch = **program;

        if (!isValidSymbolName(ch)) {
            char symbol[startOffset + 1];

            for (uint32_t i = 0; i < startOffset; i++)
                symbol[startOffset - 1 - i] = *((*program) - i - 1);
            symbol[startOffset] = '\0';

            printf("Symbol: %s\n", symbol);
            return;
        }

        startOffset++;
        (*program)++;
    }
}

Script compileScript(char *program) {
    Script script = newScript();

    while (*program) {
        char ch = *program;

        if (isCharacter(ch)) {
            parseSymbol(&script, &program);
        }

        program++;
    }

    shrinkScript(&script);
    return script;
}