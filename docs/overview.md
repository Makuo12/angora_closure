# Angora Overview

Angora consists of a fuzzer, instrumenting compilers and runtime libraries. 
Target programs should be compiled with instrumentation in order to collect
runtime information. 

Two copies of the target program should be prepared, specifically one with
taint tracking instrumentation and the other with branch and constraint 
instrumentation. This ensures a reasonable amount of efficiency when fuzzing
due to taint tracking being resource demanding. 

Similar to AFL, Angora mutates a set of seeds to increase program coverage.
Inputs that trigger new explored branches will be appended to the queue. 
Angora implements a wide selection of strategies to solve branch constraints.
For each new seed, taint tracking will be applied to learn which part of the
input will affect which branch constraint. Then mutations will be applied to
the input with the tainted parts in consideration. This allows for efficient
and precise input generation which significantly increases input coverage.


## Directory Structure

- `build`: Scripts for building Angora components.
- `common`: Common constants and data structures.
- `fuzzer`: Contains the source code for the fuzzer. The fuzzer runs the target program and repeatedly mutates the input attempting to increase its code coverage statistics.
  - `src/bin`: Source files for the executable binaries.
  - `src/depot`: Depot module for input/output file management.
  - `src/executor`: Executor module for managing target program runs.
  - `src/search`: Exploration strategies. You are free to implement and integrate your own strategy with Angora.
  - `src/cond_stmt`: Conditional statement module for constraints.
  - `src/mut_input`: Input bytes for conditional statements.
  - `src/track`: Parse taint analysis result.
  - `src/stats`: Statistical chart.
  - `src/branches`: Branch counting.
- `global_mem.c`: Manages global variable state for fuzzing. It copies initial global variable values from a special section and restores them after each target execution to ensure isolation. Includes handle_fuzz(), which calls the target's main function (renamed to target_main() via LLVM pass), handles crashes with signal jumps, and suppresses stdout/stderr.
- `angora_main_fuzz.c`: Serves as the fuzzer's entry point. Defines angora_fuzz_main() (renamed to main() at compile time via LLVM pass), which initializes Rust fuzzing logic. Also provides functions to set the comparison ID and area pointer for Angora's instrumentation.
- `llvm_mode`: Includes source code for instrumenting compilers and DFSan, the taint tracking framework.
- `llvm_mode/closure`: This directory holds the llvm passes for closure.
- `closure/src`: Closure module for housing all closure function (myMalloc, myRealloc, fopen_hook etc...) for tracking used pointers and performing clean up of those pointers on an exit of the target.
- `track_shell.sh`: Builds the target with Angora + DFSan tracking passes, producing a taint-enabled binary (build_main/pdftotext.taint).
- `fast_shell.sh`: Builds the target with Angora and closure passes, links it with Angora/closure runtime libraries and the fused fuzzer object, and produces the final fast fuzzing binary (build_main/pdftotext.fast).
- `runtime`: Taint tracking runtime library for target program.
- `runtime_fast`: Branch and constraint information collection library for target program.
- `tests`: Sample tests to evaluate fuzzer performance.
- `tools`: Some scripts.
- `docs`: Documentation.

