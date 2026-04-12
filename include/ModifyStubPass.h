#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

using namespace llvm;

struct ModifyStubPass : public ModulePass
{
    static char ID;
    ModifyStubPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
};