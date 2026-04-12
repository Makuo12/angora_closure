#include "FileHookPass.h"

using namespace llvm;

char FileHookPass::ID = 0;

void FileHookPass::hookFileRelatedCalls(Module &M)
{
    Function *fopenFunc = M.getFunction(FOPEN);
    if (fopenFunc != nullptr)
    {
        FunctionCallee fopenHook = M.getOrInsertFunction(FOPEN_HOOK, fopenFunc->getFunctionType());
        fopenFunc->replaceAllUsesWith(fopenHook.getCallee());
    }

    Function *fcloseFunc = M.getFunction(FCLOSE);
    if (fcloseFunc != nullptr)
    {
        FunctionCallee fcloseHook = M.getOrInsertFunction(FCLOSE_HOOK, fcloseFunc->getFunctionType());
        fcloseFunc->replaceAllUsesWith(fcloseHook.getCallee());
    }
}

bool FileHookPass::runOnModule(Module &M)
{
    if (isClosureStubModule(M.getName()))
        return false;

    hookFileRelatedCalls(M);
    return true;
}