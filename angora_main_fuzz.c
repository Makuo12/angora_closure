#include <stdio.h>
#include <stdlib.h>
extern void rust_fuzz_init(void);

extern unsigned char *__angora_area_ptr;

void angora_fuzz_main()
{
	printf("Starting fuzz \n");
	rust_fuzz_init(); // calls into Rust
}


void set_angora_area_ptr(unsigned char *ptr)
{
	__angora_area_ptr = ptr;
	printf("[set_angora_area_ptr] set to %p\n", ptr);
}