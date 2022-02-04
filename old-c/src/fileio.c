#include "fileio.h"

char *getFileContents(const char *inputFile) {
    FILE *file = fopen(inputFile, "r");
    if (file == NULL) {
        fprintf(stderr, "[ERROR] File failed to open: %s\n", inputFile);
	    return NULL;
    }

    // Get length of file
    fseek(file, 0, SEEK_END);
    long fileLength = ftell(file);
    fseek(file, 0, 0);
    
    // Allocate buffer + 1 byte for '\0'
    char *fileContents = malloc((fileLength + 1) * sizeof(char));

    // Read file
    int i = -1;
    while ((fileContents[++i] = fgetc(file)) != EOF); // Copy into file_contents until EOF
    fileContents[i] = '\0'; // then add '\0' as string terminator

    fclose(file);

    return fileContents;
}