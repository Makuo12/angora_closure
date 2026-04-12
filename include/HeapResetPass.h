#pragma once

#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

using namespace llvm;

struct HeapResetPass : public ModulePass
{
    static char ID;
    HeapResetPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
    void heapManage(Module &M);
};