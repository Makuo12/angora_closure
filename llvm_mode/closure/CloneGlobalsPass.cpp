#include "CloneGlobalsPass.h"
#include "llvm/IR/GlobalValue.h"

using namespace llvm;

char CloneGlobalsPass::ID = 0;

void CloneGlobalsPass::cloneGlobals(Module &M)
{
    auto &list = M.getGlobalList();
    for (auto &Global : list)
    {
        if (!Global.hasSection() && !Global.isConstant() && !Global.isThreadLocal()) 
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