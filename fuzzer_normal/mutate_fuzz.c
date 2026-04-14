#include <stdio.h>
#include <ctype.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define NO_COV __attribute__((no_sanitize("coverage")))
// M. Jurczyk, "Effective File Format Fuzzing: Thoughts, Techniques and Results," presented at Black Hat Europe 2016, London, UK, Nov. 2016. [Online]. Available: https://blackhat.com/docs/eu-16/materials/eu-16-Jurczyk-Effective-File-Format-Fuzzing-Thoughts-Techniques-And-Results.pdf
// S. Gan, C. Zhang, P. Chen, B. Zhao, X. Qin, D. Wu, and Z. Chen, "GREYONE: Data Flow Sensitive Fuzzing," in Proc. 29th USENIX Security Symposium (USENIX Security '20), Aug. 2020, pp. 2577–2594. [Online]. Available: https://www.usenix.org/conference/usenixsecurity20/presentation/gan

static void mutate_bitflip(unsigned char *data, size_t size);
static void mutate_byteflip(unsigned char *data, size_t size);
static size_t pick_mutation_count(size_t file_size);
static void mutate_add_subtract(unsigned char *data, size_t size);
size_t mutate(unsigned char *data, size_t size);
// Global tracking
const unsigned long long EPOCH_SIZE = 200000; // Reset every 200k test cases

extern int test_cases;

NO_COV
static void mutate_bitflip(unsigned char *data, size_t size)
{
    size_t pos = 10 + (rand() % (size - 110));
    int num_bits = (rand() % 4) + 1;
    for (int i = 0; i < num_bits; i++)
    {
        int bit = rand() % 8;
        data[pos] ^= (1 << bit);
    }
}

NO_COV
static void mutate_byteflip(unsigned char *data, size_t size)
{
    size_t pos = rand() % size;
    data[pos] = rand() % 256;
}

NO_COV
static void mutate_special_int(unsigned char *data, size_t size)
{
    uint32_t special[] = {
        0x00000000,
        0xFFFFFFFF,
        0x7FFFFFFF,
        0x80000000,
        0x00000001,
        0x00000100,
        0x0000FFFF,
        0xFFFFFFFE};
    if (size < 4)
        return;
    size_t pos = 10 + (rand() % (size - 110));
    uint32_t val = special[rand() % 8];
    data[pos] = (val) & 0xFF;
    data[pos + 1] = (val >> 8) & 0xFF;
    data[pos + 2] = (val >> 16) & 0xFF;
    data[pos + 3] = (val >> 24) & 0xFF;
}

NO_COV
static void mutate_chunk_spew(unsigned char *data, size_t size)
{
    if (size < 8)
        return;
    size_t src = rand() % (size / 2);
    size_t dst = rand() % (size / 2) + (size / 2);
    size_t len = (rand() % 16) + 1;
    if (src + len >= size)
        len = size - src - 1;
    if (dst + len >= size)
        len = size - dst - 1;
    memcpy(data + dst, data + src, len);
}

NO_COV
static void mutate_add_subtract(unsigned char *data, size_t size)
{
    size_t pos = 10 + (rand() % (size - 110));
    int delta = (rand() % 70) - 35;
    data[pos] = (unsigned char)(data[pos] + delta);
}

NO_COV
static size_t pick_mutation_count(size_t file_size)
{
    unsigned long long current_epoch_exec = test_cases % EPOCH_SIZE;
    float ratio;
    if (current_epoch_exec < (EPOCH_SIZE * 0.1))
    {
        ratio = 0.0001f;
    }
    else if (current_epoch_exec < (EPOCH_SIZE * 0.2))
    {
        ratio = 0.0005f;
    }
    else if (current_epoch_exec < (EPOCH_SIZE * 0.4))
    {
        ratio = 0.001f;
    }
    else if (current_epoch_exec < (EPOCH_SIZE * 0.6))
    {
        ratio = 0.005f;
    }
    else if (current_epoch_exec < (EPOCH_SIZE * 0.8))
    {
        ratio = 0.0015f;
    }
    else
    {
        ratio = 0.02f;
    }
    float micro_ramp = (float)current_epoch_exec / (float)EPOCH_SIZE * 0.005f;
    size_t count = (size_t)(file_size * (ratio + micro_ramp));
    if (count < 2)
        return 2;
    if (count > 512)
        return 512;
    return count;
}

NO_COV
size_t mutate(unsigned char *data, size_t size)
{
    size_t mutations = pick_mutation_count(size);
    for (size_t i = 0; i < mutations; i++)
    {
        int strategy = rand() % 5;
        switch (strategy)
        {
        case 0:
            mutate_bitflip(data, size);
            break;
        case 1:
            mutate_byteflip(data, size);
            break;
        case 2:
            mutate_chunk_spew(data, size);
            break;
        case 3:
            mutate_special_int(data, size);
            break;
        case 4:
            mutate_add_subtract(data, size);
            break;
        }
    }
    return size;
}