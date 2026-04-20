#include <stdio.h>
#include <stdlib.h>
extern void rust_fuzz_init(int argc, char *argv[]);

extern unsigned char *__angora_area_ptr;

extern unsigned int __angora_cond_cmpid;

void angora_fuzz_main(int argc, char *argv[])
{
	printf("Starting fuzz \n");
	rust_fuzz_init(argc, argv); // calls into Rust
}

void set_angora_cmpid(unsigned int id)
{
	__angora_cond_cmpid = id;
}

void set_angora_area_ptr(unsigned char *ptr)
{
	__angora_area_ptr = ptr;
	printf("[set_angora_area_ptr] set to %p\n", ptr);
}