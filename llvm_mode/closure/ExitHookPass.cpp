#include "ExitHookPass.h"
#include "llvm/IR/DerivedTypes.h"

using namespace llvm;

char ExitHookPass::ID = 0;

void ExitHookPass::hookExit(Module &M)
{
    auto exitFunc = M.getFunction("exit");
    if (exitFunc != nullptr)
    {
        FunctionCallee exitHook = M.getOrInsertFunction("exitHook", exitFunc->getFunctionType());
        exitFunc->replaceAllUsesWith(exitHook.getCallee());
    }
}

bool ExitHookPass::runOnModule(Module &M)
{
    if (M.getName().contains("closure_stub"))
        return false;

    hookExit(M);
    return true;
}