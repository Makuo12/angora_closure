#include <stdio.h>
extern void rust_fuzz_init(void);

void angora_fuzz_main()
{
	printf("Starting fuzz \n");
	rust_fuzz_init(); // calls into Rust
}