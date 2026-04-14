
#ifndef COMMAND_FUZZ_H_
#define COMMAND_FUZZ_H_

#include <stdio.h>

typedef struct
{
    unsigned char *data;
    size_t size;
    size_t max_size;
} Buffer;

#endif