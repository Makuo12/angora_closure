#ifndef MAIN_FUZZ_H_
#define MAIN_FUZZ_H_

#include <time.h>
#include <unistd.h>
#include <sys/mman.h>
#include <stdlib.h>
#include <stdio.h>
#include <dirent.h>
#include <fcntl.h>
#include <string.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/stat.h>
#include "command_fuzz.h"
size_t mutate(unsigned char *data, size_t size);
void update_seed(unsigned char *buffer, size_t size, int index);


#endif