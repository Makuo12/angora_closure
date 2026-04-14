#include <string.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <time.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/wait.h>
#include <sys/stat.h>
#include "command_fuzz.h"

extern int test_cases;
extern Buffer * main_buffer;
extern int main_counter;
extern int main_fix_counter;
extern int main_size;
extern char normal_out[];

#define NO_COV __attribute__((no_sanitize("coverage")))

int interesting_seed_count = 0;
int update_seed_counter = 1;
void write_interesting_seed(unsigned char *buffer, size_t size, int index);
void update_seed(unsigned char * buffer, size_t size, int index);

NO_COV
void handle_crash(unsigned char *buffer, size_t size, int index)
{
    if (index < 0)
    {
        return;
    }
    if (main_counter > index)
    {
        Buffer buf_data = main_buffer[index];
        if (buf_data.data != NULL)
        {
            // Write crashed buffer to normal_out/crashes/
            char crash_dir[1024];
            snprintf(crash_dir, sizeof(crash_dir), "%s/crashes", normal_out);
            mkdir(normal_out, 0755);
            mkdir(crash_dir, 0755);

            char crash_path[1024];
            snprintf(crash_path, sizeof(crash_path), "%s/crash_%d_%zu.bin", crash_dir, index, size);
            int fd = open(crash_path, O_CREAT | O_WRONLY | O_TRUNC, 0666);
            if (fd >= 0)
            {
                write(fd, buf_data.data, buf_data.size);
                close(fd);
            }
            free(buf_data.data);
            buf_data.data = NULL;
            if (main_counter == 0)
            {
                main_buffer[index].data = NULL;
                main_buffer[index].size = 0;
                main_buffer[index].max_size = 0;
                main_counter--;
                return;
            }
            Buffer last_buff = main_buffer[main_counter - 1];
            unsigned char *new_buf = (unsigned char *)realloc(last_buff.data, last_buff.size + 100);
            if (new_buf == NULL)
            {
                main_buffer[index].data = last_buff.data;
                main_buffer[index].size = last_buff.size;
                main_buffer[index].max_size = last_buff.max_size;
            }
            else
            {
                Buffer data = {new_buf, last_buff.size, last_buff.size + 100};
                main_buffer[index] = data;
            }
            main_buffer[main_counter - 1].data = NULL;
            main_buffer[main_counter - 1].size = 0;
            main_buffer[main_counter - 1].max_size = 0;
            main_counter--;
        }
    }
}

NO_COV
void update_seed(unsigned char *buffer, size_t size, int index)
{
    if (buffer == NULL)
    {
        fprintf(stderr, "Passed NULL buffer to update_seed\n");
        exit(1);
    }
    int extra = (int)((90.0 / 100.0) * main_fix_counter);
    int total = main_fix_counter + extra;
    if (main_counter >= main_size)
    {
        main_size *= 2;
        Buffer *new_buffer = (Buffer *)realloc(main_buffer, main_size * sizeof(Buffer));
        if (new_buffer == NULL)
        {
            perror("realloc failed in update_seed");
            exit(1);
        }
        main_buffer = new_buffer;
    }
    if (main_counter >= total)
    {
        int range = main_counter - main_fix_counter;
        if (range <= 0)
        {
            fprintf(stderr, "update_seed: no replaceable entries available\n");
            return;
        }
        int random_index = main_fix_counter + (rand() % range);
        Buffer *target = &main_buffer[random_index];
        unsigned char *new_data = (unsigned char *)realloc(target->data, size);
        if (new_data == NULL)
        {
            perror("realloc in update_seed failed");
            exit(1);
        }
        target->data = new_data;
        target->size = size;
        memcpy(target->data, buffer, size);
    }
    else
    {
        unsigned char *new_item = (unsigned char *)malloc(size * 2);
        if (new_item == NULL)
        {
            perror("malloc failed in update_seed");
            exit(1);
        }
        memcpy(new_item, buffer, size);
        Buffer b = {new_item, size, size * 2};
        main_buffer[main_counter++] = b;
    }
    write_interesting_seed(buffer, size, index);
}

NO_COV
void write_interesting_seed(unsigned char *buffer, size_t size, int index)
{
    char dir_path[1024];
    snprintf(dir_path, sizeof(dir_path), "%s/interesting", normal_out);
    mkdir(normal_out, 0755);
    mkdir(dir_path, 0755);

    char filename[1024];
    snprintf(filename, sizeof(filename), "%s/seed_%d.pdf", dir_path, index);
    int fd = open(filename, O_CREAT | O_RDWR | O_TRUNC, 0644);
    if (fd < 0)
    {
        perror("failed to open interesting seed file");
        return;
    }
    ftruncate(fd, size);
    void *ptr = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED)
    {
        perror("mmap");
        close(fd);
        return;
    }
    memcpy(ptr, buffer, size);
    msync(ptr, size, MS_SYNC);
    munmap(ptr, size);
    close(fd);

    // Write timer to normal_out/timer.txt
    char timer_path[1024];
    snprintf(timer_path, sizeof(timer_path), "%s/timer.txt", normal_out);
    FILE *timer_file = fopen(timer_path, "w");
    if (timer_file == NULL)
    {
        perror("failed to open timer.txt");
        return;
    }
    time_t now = time(NULL);
    fprintf(timer_file, "test_cases: %d\ntime: %s", test_cases, ctime(&now));
    fclose(timer_file);
}