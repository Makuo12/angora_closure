#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

using namespace llvm;

struct ExitHookPass : public ModulePass
{
    static char ID;
    ExitHookPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
    void hookExit(Module &M);
};