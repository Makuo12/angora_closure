#include "CloneGlobalsPass.h"
#include "llvm/IR/GlobalValue.h"

using namespace llvm;

char CloneGlobalsPass::ID = 0;

void CloneGlobalsPass::cloneGlobals(Module &M)
{
    for (auto &Global : M.globals())
    {
        // Only touch definitions, not external declarations
        if (!Global.isDeclaration() &&
            !Global.isConstant() &&
            !Global.isThreadLocal() &&
            !Global.hasSection())
        {
            Global.setSection(CLOSURE_GLOBAL_SECTION);
        }
    }
}

bool CloneGlobalsPass::runOnModule(Module &M)
{
    if (isClosureStubModule(M.getName()))
        return false;

    cloneGlobals(M);
    return true;
}