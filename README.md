# Angora Closure Fuzzer

## Overview

**Angora Closure** is a project that combines the effectiveness and efficiency of the Angora taint tracking algorithm while replacing its fork server (used for fast mode execution) with a persistent-style fuzzer.

This is achieved using an LLVM pass that replaces the target program’s `main` function with a custom-defined entry point. The custom `main` directly invokes the Angora fuzzing loop and replaces the fork server mechanism with direct calls to the target’s original `main`, enabling repeated execution of test cases within a single process.

To address inconsistent program states caused by persistent fuzzing, this project introduces **ClosureX**. ClosureX logs changes to the program state during execution (e.g., modified global variables) and restores them before executing the next test case.

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

* It is only invoked occasionally
* It has minimal performance impact

---

## Usage

### 1. Angora Closure Fuzzer

#### Build and Start Container

```bash
docker compose up xpdf-angora -d
docker compose exec xpdf-angora /bin/bash
```

#### Inside the Container

```bash
./angora_shell.sh
```

This will:

* Build both the **fast binary** and the **track binary**
* Start the fuzzer

#### Output

Results are stored in:

``` bash
angora_out
```

---

### 2. Fuzzer with Closure (No Angora)

This project can also run without Angora using a standard mutation-based fuzzer.

Mutation strategies include:

* Bit flips
* Byte flips
* Add/subtract operations

This version still uses a **persistent fuzzer**, and ClosureX is used to maintain consistent program state.

#### Build and Start Container (Normal-Based)

```bash
docker compose up xpdf-normal -d
docker compose exec xpdf-normal /bin/bash
```

#### Inside the Container (Normal-Based)

```bash
cd build_normal && ./pdtotext /tmp/normal_out/source.pdf
```

This will:

* Build required binaries
* Start the fuzzer

#### Output (Normal-Based)

Results are stored in:

``` bash
/tmp/normal_out
```

---

## Summary

* Combines Angora’s taint tracking with a persistent fuzzing model
* Introduces ClosureX for program state restoration
* Supports both:

  * Angora-based fuzzing
  * Standalone mutation-based fuzzing

## Limitations

It currently only works with the xpdf source code. Using other source code would require configuration changes to the shell scripts.

## References

* **Angora: Efficient Fuzzing by Principled Search**  
  Peng Chen, Hao Chen  
  [Read Paper](https://web.cs.ucdavis.edu/~hchen/paper/chen2018angora.pdf)

* **ClosureX: Compiler Support for Correct Persistent Fuzzing**  
  Rishi Ranjan, Ian Paterson, Matthew Hicks  
  [Read Paper](https://dl.acm.org/doi/pdf/10.1145/3669940.3707281)
