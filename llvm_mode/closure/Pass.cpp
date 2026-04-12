#include "llvm/IR/LegacyPassManager.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"
#include "RenameMainPass.h"
#include "HeapResetPass.h"
#include "ExitHookPass.h"
#include "CloneGlobalsPass.h"
#include "FileHookPass.h"

using namespace llvm;

// Legacy pass registration
static void registerPasses(const PassManagerBuilder &, legacy::PassManagerBase &PM)
{
    PM.add(new RenameMainPass());
    PM.add(new HeapResetPass());
    PM.add(new ExitHookPass());
    PM.add(new CloneGlobalsPass());
    PM.add(new FileHookPass());
}

static RegisterStandardPasses RegisterPassesOpt(
    PassManagerBuilder::EP_OptimizerLast, // runs at -O1 and above
    registerPasses);

static RegisterStandardPasses RegisterPassesO0(
    PassManagerBuilder::EP_EnabledOnOptLevel0, // runs even at -O0
    registerPasses);