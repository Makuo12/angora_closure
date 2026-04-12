#pragma once

#pragma once
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "Common.h"

#define FOPEN "fopen"
#define FOPEN_HOOK "fopen_hook"

#define FCLOSE "fclose"
#define FCLOSE_HOOK "fclose_hook"


using namespace llvm;

struct FileHookPass : public ModulePass
{
    static char ID;
    FileHookPass() : ModulePass(ID) {}
    bool runOnModule(Module &M) override;
    void hookFileRelatedCalls(Module &M);
};