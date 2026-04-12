#include "RenameMainPass.h"
#include "llvm/IR/Function.h"
#include <cstdlib>

using namespace llvm;

char RenameMainPass::ID = 0;

#define FUZZER_TYPE "FUZZER_TYPE"

bool RenameMainPass::runOnModule(Module &M)
{
    if (isClosureStubModule(M.getName()))
        return false;

    const char *fuzzer_type = getenv(FUZZER_TYPE);
    bool is_normal = (fuzzer_type != nullptr && strcmp(fuzzer_type, "normal") == 0);
    bool Changed = false;

    for (auto &F : M)
    {
        if (F.getName() == ENTRYPOINT_NAME)
        {
            F.setName("target_main");
            Changed = true;
            break;
        }

        llvm::StringRef entrypoint = is_normal ? NORMAL_ENTRYPOINT : ANGORA_ENTRYPOINT;
        if (F.getName() == entrypoint)
        {
            F.setName("main");
            Changed = true;
            break;
        }
    }
    return Changed;
}