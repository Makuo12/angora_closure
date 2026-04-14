#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/stat.h>
#include "coverage_fuzz.h"
#include <sanitizer/coverage_interface.h>

char *start_ptr;
char *end_ptr;
void read_coverage(char[]);
int handle_coverage(void);
void realloc_handler(long long **item, size_t size);
static int coverage_loaded = 0;

static long long *numbers;
static long long number_count = 0;
static long long number_size = 128;

static char normal_out[] = "/tmp/normal_out";

int current_region = 0;
int region_size = 0;
Region *regions = NULL;

#define NO_COV __attribute__((no_sanitize("coverage")))

void __sanitizer_cov_8bit_counters_init(char *start, char *end)
{
	if (start == end)
		return;
	if (regions == NULL)
	{
		region_size = 128;
		Region *r = (Region *)malloc(region_size * sizeof(Region));
		if (r == NULL)
		{
			perror("malloc");
			exit(1);
		}
		regions = r;
	}
	if (current_region >= region_size)
	{
		region_size *= 2;
		Region *ptr = (Region *)realloc(regions, region_size * sizeof(Region));
		if (ptr == NULL)
		{
			perror("No memory avaiable");
			return;
		}
		regions = ptr;
	}
	regions[current_region].start_ptr = (unsigned char *)start;
	regions[current_region].end_ptr = (unsigned char *)end;
	current_region++;
	if (numbers == NULL)
	{
		numbers = (long long *)malloc(number_size * sizeof(long long));
		if (numbers == NULL)
		{
			perror("malloc numbers");
			exit(1);
		}
	}
}

NO_COV void realloc_handler(long long **item, size_t size)
{
	long long *new_item = (long long *)realloc(*item, size);
	if (new_item == NULL)
	{
		perror("realloc");
		exit(1);
	}
	*item = new_item;
}

NO_COV void read_coverage(char file_path[])
{
    if (coverage_loaded) return;
    coverage_loaded = 1;

    char *line = NULL;
    size_t len = 0;
    FILE *fp = fopen(file_path, "r");
    if (fp)
    {
        while ((getline(&line, &len, fp)) != -1)
        {
            if (number_count >= number_size)
            {
                number_size *= 2;
                realloc_handler(&numbers, number_size * sizeof(long long));
            }
            numbers[number_count++] = atoll(line);
            free(line);
            line = NULL;
        }
        fclose(fp);
    }
}

NO_COV
int handle_coverage(void)
{
	mkdir(normal_out, 0777);
	char file_path[1024];
	snprintf(file_path, sizeof(file_path), "%s/coverage.txt", normal_out);
	read_coverage(file_path);
	long long count = 1;
	long long hits = 0;
	for (int i = 0; i < current_region; i++)
	{
		unsigned char *ptr = regions[i].start_ptr;
		unsigned char *end = regions[i].end_ptr;
		// Process counters from ptr to end...
		for (unsigned char *p = ptr; p < end; p++)
		{
			unsigned char value = (unsigned char)*p;
			hits += count * value;
			count++;
		}
	}
	int greater_than = 0;
	for (int i = 0; i < number_count; i++)
	{
		if (hits > numbers[i])
		{
			greater_than++;
		}
	}
	if (greater_than > 0)
	{
		FILE *fp = fopen(file_path, "a");
		if (fp)
		{
			fprintf(fp, "%lld\n", hits);
			fclose(fp);
			if (number_count >= number_size)
			{
				number_size *= 2;
				realloc_handler(&numbers, number_size * sizeof(long long));
			}
			numbers[number_count++] = hits;
			return 1;
		}
	}
	return 0;
}
