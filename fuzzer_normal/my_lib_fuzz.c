
#include "my_lib_fuzz.h"
#define NO_COV __attribute__((no_sanitize("coverage")))

Buffer *main_buffer = NULL;
int main_counter = 0;
int main_fix_counter = 0;
int main_size = 128;
int test_cases = 0;

extern void handle_closure_init(void);
extern void handle_closure_reset(void);
extern int handle_fuzz(int argc, char *argv[]);
extern void handle_crash(unsigned char *buffer, size_t size, int index);
extern int handle_coverage(void);
char normal_out[] = "/tmp/normal_out";

NO_COV
static Buffer handle_file(char *filename)
{
	Buffer buf = {NULL, 0, 0};
	struct stat st;
	if ((stat(filename, &st)) != 0)
	{
		perror("stat");
		return buf;
	}
	size_t size = st.st_size;
	if (size == 0)
		return buf;
	FILE *fp = fopen(filename, "rb");
	if (fp == NULL)
	{
		return buf;
	}
	size_t max_size = 2 * size;
	unsigned char *buffer = (unsigned char *)malloc(max_size);
	size_t bytes_read = fread(buffer, sizeof(unsigned char), size, fp);
	if (bytes_read < 1)
	{
		return buf;
	}
	buf.data = buffer;
	buf.size = bytes_read;
	buf.max_size = max_size;
	fclose(fp);
	return buf;
}

NO_COV
static int is_pdf(const char *name)
{
	size_t len = strlen(name);
	return (len > 4 && strcmp(name + len - 4, ".pdf") == 0);
}

#include <dirent.h>

NO_COV
static void init_file()
{
	struct dirent **namelist;
	// scandir filters using our custom 'is_pdf' and sorts alphabetically
	char buffer[1024];
	if (getcwd(buffer, sizeof(buffer)) == NULL)
	{
		return;
	}
	char file_path[1024];
	snprintf(file_path, sizeof(file_path), "%s/../pdf", buffer);
	int n = scandir(file_path, &namelist, NULL, alphasort);
	if (n < 0)
	{
		perror("scandir");
		return;
	}

	for (int i = 0; i < n; i++)
	{
		if (is_pdf(namelist[i]->d_name))
		{
			char path[2024];
			snprintf(path, sizeof(path), "%s/../pdf/%s", buffer, namelist[i]->d_name);
			Buffer b = handle_file(path);
			if (b.data)
				main_buffer[main_counter++] = b;
		}
		free(namelist[i]); // Clean up as we go
	}
	free(namelist);
	main_fix_counter = main_counter;
}

NO_COV time_t init_main()
{
	time_t start = time(NULL);
	printf("start: %ld\n", start);
	srand(start);
	Buffer *b = (Buffer *)malloc(main_size * sizeof(Buffer));
	if (b == NULL)
	{
		perror("malloc");
		exit(1);
	}
	main_buffer = b;
	init_file();
	return start;
}

NO_COV
void setup_time(time_t start)
{
	time_t end = time(NULL);
	double elapsed = difftime(end, start);
	if ((int)(elapsed / 60) % 5 == 0 && (int)elapsed % 60 == 0 && elapsed > 0)
	{
		printf("%.0f minutes elapsed. Total test cases executed so far: %d\n", elapsed / 60, test_cases);
	}
	if (elapsed > 86400)
	{
		printf("24 hours elapsed, exiting...\n");
		printf("Total test cases executed: %d\n", test_cases);
		exit(0);
	}
}

NO_COV
Buffer apply_mutate(int index)
{
	test_cases++;
	unsigned char *buffer_mutate = (unsigned char *)malloc(main_buffer[index].max_size);
	if (buffer_mutate == NULL)
	{
		perror("malloc");
		exit(1);
	}
	memcpy(buffer_mutate, main_buffer[index].data, main_buffer[index].size);
	size_t new_size = mutate(buffer_mutate, main_buffer[index].size);

	mkdir(normal_out, 0777);
	char file_path[1024];
	snprintf(file_path, sizeof(file_path), "%s/source.pdf", normal_out); // ← fixed

	int fd = open(file_path, O_CREAT | O_RDWR | O_TRUNC, 0644);
	if (fd < 0)
	{
		perror("open");
		exit(1);
	}
	ftruncate(fd, new_size);
	void *ptr = mmap(NULL, new_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if (ptr == MAP_FAILED) // ← fixed
	{
		perror("mmap");
		exit(1);
	}
	memcpy(ptr, buffer_mutate, new_size);
	msync(ptr, new_size, MS_SYNC);
	munmap(ptr, new_size);
	close(fd);

	Buffer b = {buffer_mutate, new_size, main_buffer[index].max_size};
	return b;
}

NO_COV
void handle_seed(Buffer *b, int index)
{
	update_seed(b->data, b->size, index);
}

NO_COV
void free_seed(Buffer *b)
{
	free(b->data);
}

NO_COV
void my_caller(int argc, char *argv[]) {
	time_t start = init_main();
    handle_closure_init();
	while (1)
	{
		setup_time(start);
		for (int index = 0; index < main_counter; index++)
		{
			Buffer b = apply_mutate(index);
			int result = handle_fuzz(argc, argv);
			if (result > 1) {
				// We know there is a crash;
				handle_crash(b.data, b.size, index);
			} else {
				int found = handle_coverage();
				if (found) {
					update_seed(b.data, b.size, index);
				}
			}
			handle_closure_reset();
			free_seed(&b);
		}
	}
}