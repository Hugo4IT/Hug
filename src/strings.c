#include "strings.h"

bool startsWith(char *haystack, char *needle) {
    int i = 0;
    while (true) {
        if (needle[i] == '\0') return true;
        if (haystack[i] == '\0') return false;
        if (haystack[i] != needle[i]) return false;
        i++;
    }
}

bool stringEquals(char *left, char *right) {
    int i = 0;
    while(right[i] == left[i])
        if (left[i++] == '\0') return false;
    return true;
}