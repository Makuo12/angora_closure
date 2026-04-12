#pragma once

#include <llvm/IR/PassManager.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/PassPlugin.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/IR/DebugInfoMetadata.h>
#include <llvm/IR/DebugLoc.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/GlobalVariable.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Type.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/Support/SpecialCaseList.h>
#include <llvm/IR/Dominators.h>
#include <llvm/IR/InstVisitor.h>
#include "llvm/Support/VirtualFileSystem.h" // vfs::getRealFileSystem(), vfs::InMemoryFileSystem, etc.
#include "llvm/IR/MDBuilder.h"
#include "llvm/IR/Attributes.h"
#include "llvm/Support/FileSystem.h"        // sys::fs::* (real filesystem ops)
#include "llvm/Transforms/Utils/Local.h"
#include "llvm/Transforms/Utils/BasicBlockUtils.h"
#include "llvm/IR/Attributes.h"
#include "llvm/IR/Dominators.h"                    // DominatorTree
#include "llvm/Analysis/DomTreeUpdater.h"          // DomTreeUpdater
#include "llvm/Transforms/Utils/BasicBlockUtils.h" // SplitBlockAndInsertIfThen
#include "llvm/IR/Attributes.h"
#include "llvm/IR/Function.h"

#define STUB_FILE_NAME_PREFIX "stubMain"
#define TARGET_MAIN_FUNC "start_main"
#define ENTRYPOINT_NAME "main"
#define ANGORA_ENTRYPOINT "angora_fuzz_main"
#define NORMAL_ENTRYPOINT "handle_main"
#define CLOSURE_GLOBAL_RESTORE_FILE "/tmp/CLOSURE_GLOBAL_RESTORE_FILE"

/**
 * @brief Checks if the module being compiled is the closure stub
 *
 * @param moduleName
 * @return true
 * @return false
 */
bool isClosureStubModule(llvm::StringRef);

std::string generateRandomString(int len);
