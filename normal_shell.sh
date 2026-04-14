#!/bin/bash
set -euxo pipefail  # Fail fast if any command fails

# 1. Use absolute paths based on the script location
# This ensures ANGORA_PATH is always /angora_closure/bin regardless of 'cd'
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
ANGORA_PATH="${SCRIPT_DIR}/bin"
FUZZ_MAIN_A="${SCRIPT_DIR}/normal_main_fuzz.a"

# 2. Build the helper objects
cd "${SCRIPT_DIR}/fuzzer_normal"
make coverage_fuzz.o
make merged.o

# 3. Build the instrumented main object
# We use the absolute path to closure.so here
cd "${SCRIPT_DIR}"
/clang+llvm/bin/clang \
        -Xclang -load -Xclang "${ANGORA_PATH}/pass/closure.so" \
        -pie -fpic -g -O3 \
        -I"${SCRIPT_DIR}/fuzzer_normal/include" \
        -c -o normal_main_fuzz.o ./fuzzer_normal/main_fuzz.c 

# 4. Link everything into a static library
ld -r \
    normal_main_fuzz.o \
    "${ANGORA_PATH}/lib/libclosure.a" \
    "./fuzzer_normal/merged.o" \
    -o normal_main_fuzz_merged.o

ar rcs normal_main_fuzz.a normal_main_fuzz_merged.o

# 5. Build xpdf
echo "Building xpdf fast binary..."
cd "${SCRIPT_DIR}/xpdf-4.06_2"
mkdir -p build_normal && cd build_normal

PASS_FLAGS="\
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/closure.so \
        -pie -fpic -Qunused-arguments \
        -fsanitize-coverage=inline-8bit-counters \
        -g -O3 -funroll-loops"

# Note: We use ${SCRIPT_DIR} for relative paths to ensure they work inside 'build_normal'
LINK_FLAGS="\
    -Wl,--allow-multiple-definition \
    -Wl,--whole-archive \
        ${ANGORA_PATH}/lib/libclosure.a \
    -Wl,--no-whole-archive \
    ${FUZZ_MAIN_A} \
    -Wl,--no-as-needed \
    -Wl,--gc-sections \
    ${SCRIPT_DIR}/fuzzer_normal/coverage_fuzz.o \
    -ldl -lpthread -lm"

cmake -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_C_COMPILER=/clang+llvm/bin/clang \
        -DCMAKE_CXX_COMPILER=/clang+llvm/bin/clang++ \
        -DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY \
        -DCMAKE_C_FLAGS="${PASS_FLAGS}" \
        -DCMAKE_CXX_FLAGS="${PASS_FLAGS}" \
        -DCMAKE_EXE_LINKER_FLAGS="${LINK_FLAGS}" \
        -DCMAKE_SHARED_LINKER_FLAGS="${LINK_FLAGS}" \
        ..

make pdftotext -j$(nproc)

# 6. Final cleanup
cd "${SCRIPT_DIR}"
mkdir -p build_normal
cp ./xpdf-4.06_2/build_normal/xpdf/pdftotext build_normal/