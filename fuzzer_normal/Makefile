# Add -D_GNU_SOURCE or -std=c99 to ensure standard types are mapped correctly
CFLAGS = -I./include -O3 -fPIC -Wall -std=gnu99

CC = /clang+llvm/bin/clang

# Define the objects to keep the Makefile clean
OBJS = mutate_fuzz.o command_fuzz.o my_lib_fuzz.o global_normal_fuzz.o

# The primary targets called by your shell script
all: coverage_fuzz.o merged.o

mutate_fuzz.o: mutate_fuzz.c
	$(CC) $(CFLAGS) -c -o $@ $<

command_fuzz.o: command_fuzz.c
	$(CC) $(CFLAGS) -c -o $@ $<

my_lib_fuzz.o: my_lib_fuzz.c
	$(CC) $(CFLAGS) -c -o $@ $<

global_normal_fuzz.o: global_normal_fuzz.c
	$(CC) $(CFLAGS) -c -o $@ $<

merged.o: $(OBJS)
	ld -r $(OBJS) -o $@

coverage_fuzz.o: coverage_fuzz.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f *.o merged.o coverage_fuzz.o