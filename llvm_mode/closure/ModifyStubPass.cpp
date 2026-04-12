#include "ModifyStubPass.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/Instructions.h"
#include "llvm/Support/raw_ostream.h"
#include <fstream>
#include <string>

using namespace llvm;

char ModifyStubPass::ID = 0;

bool ModifyStubPass::runOnModule(Module &M)
{
    errs() << "Running stub modify pass for: " << M.getName() << "\n";

    Function *mainFunc = M.getFunction("main");
    if (!mainFunc)
        return false;

    Instruction *insertPt = nullptr;
    for (auto &BB : *mainFunc)
    {
        for (auto &I : BB)
        {
            if (auto *call = dyn_cast<CallInst>(&I))
            {
                Function *fp = call->getCalledFunction();
                StringRef fname = "";
                if (fp)
                {
                    fname = fp->getName();
                }
                else
                {
                    Value *sv = call->getCalledOperand()->stripPointerCasts();
                    fname = sv->getName();
                }
                if (fname == "start_main")
                {
                    insertPt = &I;
                    break;
                }
            }
        }
        if (insertPt)
            break;
    }

    if (!insertPt)
        return false;

    std::ifstream f(CLOSURE_GLOBAL_RESTORE_FILE);
    if (!f.is_open())
        return false;

    std::string funcName;
    bool changed = false;
    while (std::getline(f, funcName))
    {
        if (funcName.empty())
            continue;

        FunctionCallee restoreGlobalsFunc = M.getOrInsertFunction(
            funcName,
            FunctionType::get(Type::getVoidTy(M.getContext()), false));

        CallInst::Create(
            restoreGlobalsFunc.getFunctionType(),
            restoreGlobalsFunc.getCallee(),
            {},
            "",
            insertPt);
        changed = true;
    }
    return changed;
}