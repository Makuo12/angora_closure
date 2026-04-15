#!/bin/bash
ANGORA_PATH="$(pwd)"/bin
FUZZ_MAIN_A="$(pwd)"/angora_main_fuzz.a

# Compile angora_main_fuzz.c with fast-mode passes + closure.so
/clang+llvm/bin/clang \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/closure.so \
        -pie -fpic -g -O3 \
        -c -o angora_main_fuzz.o ./angora_main_fuzz.c 

ld -r \
        angora_main_fuzz.o \
        ${ANGORA_PATH}/lib/libclosure.a \
        -o angora_main_fuzz_merged.o

ar rcs angora_main_fuzz.a angora_main_fuzz_merged.o

echo "Building xpdf fast binary..."
cd xpdf-4.06_2
mkdir -p build_fast && cd build_fast

PASS_FLAGS="\
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/libUnfoldBranchPass.so \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/libAngoraPass.so \
        -mllvm -angora-dfsan-abilist=${ANGORA_PATH}/rules/angora_abilist.txt \
        -mllvm -angora-dfsan-abilist=${ANGORA_PATH}/rules/dfsan_abilist.txt \
        -mllvm -angora-exploitation-list=${ANGORA_PATH}/rules/exploitation_list.txt \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/closure.so \
        -pie -fpic -Qunused-arguments \
        -g -O3 -funroll-loops"

LINK_FLAGS="\
	-Wl,--allow-multiple-definition  \
        -stdlib=libc++ \
        -L${ANGORA_PATH}/lib/libcxx_fast/ \
        -lc++fast -Wl,--start-group -lc++abifast -lc++abi -Wl,--end-group \
        -Wl,--whole-archive \
        ${ANGORA_PATH}/lib/libruntime_fast.a \
        ${ANGORA_PATH}/lib/libclosure.a \
        -Wl,--no-whole-archive \
        ${FUZZ_MAIN_A} \
        ${ANGORA_PATH}/lib/libangora.a \
        -Wl,--no-as-needed \
        -Wl,--gc-sections \
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
cd ../..

mkdir -p build_fast
cp ./xpdf-4.06_2/build_fast/xpdf/pdftotext build_fast/pdftotext.fast