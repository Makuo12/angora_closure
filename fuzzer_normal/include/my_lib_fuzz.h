#ifndef MY_LIB_FUZZ_H_
#define MY_LIB_FUZZ_H_
#include "command_fuzz.h"
#include "main_fuzz.h"
extern Buffer * main_buffer;
extern int main_counter;
extern int main_fix_counter;
extern int main_size;
extern int test_cases;
void my_caller(int argc, char *argv[]);
void free_seed(Buffer *b);
void handle_seed(Buffer *b, int index);
Buffer apply_mutate(int index);
void crash_handler(int sig);
time_t init_main();
void setup_time(time_t start);
#ifdef __cplusplus
extern "C"
{
#endif

    int target_main(int argc, char *argv[]);

#ifdef __cplusplus
}
#endif 
#endif