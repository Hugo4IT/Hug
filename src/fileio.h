#pragma once

#include <stdio.h>
#include <stdlib.h>

/// Get contents from file path
/// 
/// @warning Remember to free the contents after usage
/// @throw Can throw error if path doesn't exist
char *getFileContents(const char *inputFile);