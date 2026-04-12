#ifdef __APPLE__
#include <mach-o/getsect.h>
#include <mach-o/dyld.h>
extern const struct mach_header_64 _mh_execute_header;
#endif

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdatomic.h>
#include <sys/mman.h>
#include <unistd.h>
#include <setjmp.h>
#include <signal.h>

extern int target_main(int argc, char *argv[]);

#define CLOSURE_GLOBAL_SECTION_ADDR_ENV "CLOSURE_GLOBAL_SECTION_ADDR"
#define CLOSURE_GLOBAL_SECTION_SIZE_ENV "CLOSURE_GLOBAL_SECTION_SIZE"
#define CLOSURE_GLOBAL_BITMAP_FILE "CLOSURE_GLOBAL_BITMAP_FILE"
static void *copy_memory = NULL;
static unsigned long addr = 0;
static unsigned long size = 0;
static unsigned int reset_globals = 0;

#define NO_COV __attribute__((no_sanitize("coverage")))

sigjmp_buf env;
jmp_buf __longjmp_buf__;

void handle_closure_init(void)
{
#ifdef __APPLE__
    unsigned long section_size;
    uint8_t *section_addr = getsectiondata(&_mh_execute_header, "__DATA", "__cls_glob", &section_size);
    if (section_addr == NULL)
    {
        fprintf(stderr, "Error: Could not find section __DATA,__cls_glob\n");
        return;
    }
#elif __linux__
    extern char __start___cls_glob[] __attribute__((weak));
    extern char __stop___cls_glob[] __attribute__((weak));

    if (__start___cls_glob == NULL || __start___cls_glob == __stop___cls_glob)
    {
        fprintf(stderr, "Error: Could not find section .cls_glob\n");
        return;
    }
    uint8_t *section_addr = (uint8_t *)__start___cls_glob;
    unsigned long section_size = (unsigned long)(__stop___cls_glob - __start___cls_glob);
#endif

    addr = (unsigned long)section_addr;
    size = section_size;

    printf("Runtime Address: %p\n", (void *)section_addr);
    printf("Section Size:    %lu bytes\n", size);

    copy_memory = malloc(size);
    if (!copy_memory)
    {
        perror("malloc");
        return;
    }
    memcpy(copy_memory, section_addr, size);
    reset_globals = 1;
}

void restore_global_sections(char *target, char *source, int len)
{
    long pagesize = sysconf(_SC_PAGESIZE);
    void *page_start = (void *)((uintptr_t)target & ~(pagesize - 1));
    size_t protect_len = ((uintptr_t)target + len) - (uintptr_t)page_start;

    if (mprotect(page_start, protect_len, PROT_READ | PROT_WRITE) == -1)
    {
        perror("mprotect (RW)");
        return;
    }

    memcpy(target, source, len);

    // reset back to read-only
    if (mprotect(page_start, protect_len, PROT_READ) == -1)
    {
        perror("mprotect (RO)");
    }
}

void handle_closure_reset(void)
{
    if (reset_globals != 0 && addr != 0 && size != 0)
    {
        restore_global_sections((char *)addr, (char *)copy_memory, size);
    }
}

void crash_handler(int sig)
{
    switch (sig)
    {
    case SIGSEGV:
        printf("segfault\n");
        break;
    case SIGBUS:
        printf("bus error\n");
        break;
    case SIGFPE:
        printf("arithmetic error\n");
        break;
    case SIGILL:
        printf("illegal instruction\n");
        break;
    default:
        printf("unknown signal %d\n", sig);
        break;
    }
    siglongjmp(env, sig);
}

NO_COV
void set_crash_handler()
{
    signal(SIGSEGV, crash_handler);
    signal(SIGABRT, crash_handler);
    signal(SIGFPE, crash_handler);
    signal(SIGBUS, crash_handler);
    signal(SIGTRAP, crash_handler);
    signal(SIGILL, crash_handler);
}

int handle_fuzz(int argc, char *argv[])
{
    int result = 0;
    int sig = sigsetjmp(env, 1);
    if (sig != 0)
    {
        // sig contains the signal number that caused the jump
        printf("\n!!!Saved by long jump!!! signal: %d\n", sig);
        result = sig; // store the signal number as the error code
    }
    else
    {
        result = target_main(argc, argv);
    }
    return result;
}

void exitHook(int status)
{
    longjmp(__longjmp_buf__, status);
}
