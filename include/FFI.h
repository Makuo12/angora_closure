#ifndef FFI_H_
#define FFI_H_

#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C"
{
#endif 
    void myFree(void *ptr);
    void * myMalloc(int size);
    void * myCalloc(size_t num, size_t size);
    void * myRealloc(void *ptr, size_t new_size);
    void free_ptrs(void);
    int fclose_hook(FILE * fp);
    FILE * fopen_hook(const char *filename, const char *mode);
    void close_open_file_handles(void);
    // void my_caller(int argc, char *argv[]);
    // void exitHook(int status);
    // void handle_closure_init(void);
    // void handle_closure_reset(void);
    // int handle_fuzz(int argc, char *argv[]);
    // void set_crash_handler(void);
    // void angora_fuzz_main(void);
#ifdef __cplusplus
}
#endif

#endif