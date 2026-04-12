#include "llvm/ADT/StringRef.h"
#include <string>
#include <unistd.h>
#include <ctime>

#define STUB_FILE_NAME_PREFIX "stubMain"

/**
 * @brief Checks if the module name contains the stub prefix.
 * * In LLVM 11, StringRef::contains is the idiomatic way to perform this check.
 */
bool isClosureStubModule(llvm::StringRef moduleName)
{
    // Ensure STUB_FILE_NAME_PREFIX is defined (usually in Common.h)
    return moduleName.contains(STUB_FILE_NAME_PREFIX);
}

/**
 * @brief Generates a random alphanumeric string.
 * * Note: srand/rand is acceptable for basic pass naming, but ensure
 * getpid() is available (requires <unistd.h>).
 */
std::string generateRandomString(int len)
{
    // Seed using time and PID to avoid collisions in parallel builds
    // (though still not cryptographically secure)
    static bool seeded = false;
    if (!seeded)
    {
        srand((unsigned)time(NULL) * getpid());
        seeded = true;
    }

    static const char alphanum[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                                   "abcdefghijklmnopqrstuvwxyz";
    std::string randStr;
    randStr.reserve(len);

    for (int i = 0; i < len; ++i)
    {
        randStr += alphanum[rand() % (sizeof(alphanum) - 1)];
    }

    return randStr;
}