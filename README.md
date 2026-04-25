# Angora Closure Fuzzer

## Overview

**Angora Closure** is a project that combines the effectiveness and efficiency of the Angora taint tracking algorithm while replacing its fork server (used for fast mode execution) with a persistent-style fuzzer.

This is achieved using an LLVM pass that replaces the target program’s `main` function with a custom-defined entry point. The custom `main` directly invokes the Angora fuzzing loop and replaces the fork server mechanism with direct calls to the target’s original `main`, enabling repeated execution of test cases within a single process. This is implemented using the LLVM passes in llvm_mode

To address inconsistent program states caused by persistent fuzzing, this project introduces **ClosureX**. ClosureX keeps track of changes to the program state during execution (e.g., global variables, heap memory) and restores them before executing the next test case. This is implemented using the LLVM pass located in ./llvm_mode/closure

## Design

ClosureX is integrated with Angora by introducing a custom entry point that replaces the target program’s original `main`.

* The original `main` is transformed into a callable function.
* A new `main` initializes the fuzzing environment and starts the Angora fuzzing loop.

Each execution of the target function is wrapped using `setjmp`/`longjmp` to safely handle crashes (e.g., segmentation faults) without terminating the fuzzer.

After every run:

* Control returns to the Angora fuzzing loop with a result indicating **an exit code, crash, or normal execution**.
* ClosureX performs cleanup by:

  * Releasing memory
  * Closing open files
  * Restoring global state

Angora then uses this result to guide subsequent fuzzing iterations as usual.

The taint-tracking binary is still executed using Angora’s original process-based approach. This implementation is preserved because:

* It is created by calling fork()/exec()
* It is only invoked occasionally
* It has minimal performance impact

---

## Usage

To learn more about scripts files, closure pass, angora pass, check the [Overview](./docs/overview.md)

### 1. Angora Closure Fuzzer

#### Build and Start Container

```bash
docker compose up xpdf-angora -d
docker compose exec xpdf-angora /bin/bash
```

#### Inside the Container

```bash
./shell.sh
```

This will:

* Build both the **fast binary** and the **track binary**

#### Start fuzzing

-m is the Mode
-i is the input directory
-o is the output directory
-t is the path to the taint binary
-f is the path to the fast binary
RUST_LOG=trace helps to see the trace logs (info, debug)

```bash
cd /angora_closure/build_main
```

```bash
RUST_LOG=trace ./pdftotext.fast -m llvm -i ../pdf -o ./angora_out -t ./pdftotext.taint -f ./pdftotext.fast
```

#### Output

Results are stored in:

``` bash
cd ./angora_out
```

---

## Summary

* Combines Angora’s taint tracking with a persistent fuzzing model
* Introduces ClosureX for program state restoration
* Supports both:

  * Angora-based fuzzing
  * Standalone mutation-based fuzzing

## Limitations

It currently only works with the xpdf source code. Using other source code would require configuration changes to the shell scripts. Pin mode has not been tested yet.

## References

* **Angora: Efficient Fuzzing by Principled Search**  
  Peng Chen, Hao Chen  
  [Read Paper](https://web.cs.ucdavis.edu/~hchen/paper/chen2018angora.pdf)

* **ClosureX: Compiler Support for Correct Persistent Fuzzing**  
  Rishi Ranjan, Ian Paterson, Matthew Hicks  
  [Read Paper](https://dl.acm.org/doi/pdf/10.1145/3669940.3707281)
