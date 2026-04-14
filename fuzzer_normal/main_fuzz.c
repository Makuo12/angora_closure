#include "my_lib_fuzz.h"


#define NO_COV __attribute__((no_sanitize("coverage")))

NO_COV
int handle_main(int argc, char *argv[])
{
	my_caller(argc, argv);
	return 0;
}