#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

using namespace llvm;

struct RenameMainPass : public ModulePass
{
    static char ID;
    RenameMainPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
};
