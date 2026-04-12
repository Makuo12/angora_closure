#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

#define CLOSURE_GLOBAL_SECTION "__cls_glob"

using namespace llvm;

struct CloneGlobalsPass : public ModulePass
{
    static char ID;
    CloneGlobalsPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
    void cloneGlobals(Module &M);
};