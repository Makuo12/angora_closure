#include "HeapResetPass.h"

using namespace llvm;

char HeapResetPass::ID = 0;

void HeapResetPass::heapManage(Module &M)
{
    auto freeFunc = M.getFunction("free");
    auto mallocFunc = M.getFunction("malloc");
    auto callocFunc = M.getFunction("calloc");
    auto reallocFunc = M.getFunction("realloc");

    if (mallocFunc != nullptr)
    {
        auto myMalloc = M.getOrInsertFunction("myMalloc", mallocFunc->getFunctionType());
        mallocFunc->replaceAllUsesWith(myMalloc.getCallee());
    }
    if (callocFunc != nullptr)
    {
        auto myCalloc = M.getOrInsertFunction("myCalloc", callocFunc->getFunctionType());
        callocFunc->replaceAllUsesWith(myCalloc.getCallee());
    }
    if (reallocFunc != nullptr)
    {
        auto myRealloc = M.getOrInsertFunction("myRealloc", reallocFunc->getFunctionType());
        reallocFunc->replaceAllUsesWith(myRealloc.getCallee());
    }
    if (freeFunc != nullptr)
    {
        auto myFree = M.getOrInsertFunction("myFree", freeFunc->getFunctionType());
        freeFunc->replaceAllUsesWith(myFree.getCallee());
    }
}

bool HeapResetPass::runOnModule(Module &M)
{
    if (isClosureStubModule(M.getName()))
        return false;

    heapManage(M);
    return true;
}