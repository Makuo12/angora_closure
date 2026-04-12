#!/bin/bash
ANGORA_PATH="$(pwd)"/bin

echo "Building xpdf track binary..."
cd xpdf-4.06_2
mkdir -p build_track && cd build_track

PASS_FLAGS="\
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/libUnfoldBranchPass.so \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/libAngoraPass.so \
        -mllvm -TrackMode \
        -mllvm -angora-dfsan-abilist=${ANGORA_PATH}/rules/angora_abilist.txt \
        -mllvm -angora-dfsan-abilist=${ANGORA_PATH}/rules/dfsan_abilist.txt \
        -mllvm -angora-exploitation-list=${ANGORA_PATH}/rules/exploitation_list.txt \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/libDFSanPass.so \
        -mllvm -angora-dfsan-abilist2=${ANGORA_PATH}/rules/angora_abilist.txt \
        -mllvm -angora-dfsan-abilist2=${ANGORA_PATH}/rules/dfsan_abilist.txt \
        -pie -fpic -Qunused-arguments \
        -g -O3 -funroll-loops"

# No closure.so, no angora_main_fuzz.a

LINK_FLAGS="\
        -Wl,--whole-archive \
        ${ANGORA_PATH}/lib/libdfsan_rt-x86_64.a \
        ${ANGORA_PATH}/lib/libDFSanIO.a \
        ${ANGORA_PATH}/lib/libcxx_track/libc++track.a \
        ${ANGORA_PATH}/lib/libcxx_track/libc++abitrack.a \
        -Wl,--no-whole-archive \
        -Wl,--dynamic-list=${ANGORA_PATH}/lib/libdfsan_rt-x86_64.a.syms \
        ${ANGORA_PATH}/lib/libruntime.a \
        -lrt \
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

mkdir -p build_track

cp ./xpdf-4.06_2/build_track/xpdf/pdftotext build_track/