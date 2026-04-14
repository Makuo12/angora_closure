
#ifndef COVERAGE_FUZZ_H_
#define COVERAGE_FUZZ_H_

typedef struct
{
    unsigned char *start_ptr;
    unsigned char *end_ptr;
} Region;

extern int current_region;
extern Region *regions;
int handle_coverage(void);
#endif