#!/bin/sh
set -eux

MODE="llvm"

input="./input"
output="./output"

if [ "$#" -ne 1 ] || ! [ -d "$1" ]; then
    echo "Usage: $0 DIRECTORY" >&2
    exit 1
fi

rm -rf $output
name=$1

target=${name}/${name}

bin_dir=../bin/

USE_TRACK=1 ${bin_dir}/angora-clang ${target}.c -lz -o ${target}.taint
USE_FAST=1 ${bin_dir}/angora-clang ${target}.c -lz -o ${target}_main.fast

FUZZ_MAIN_A=../angora_main_fuzz.a
ANGORA_PATH=../bin

/clang+llvm/bin/clang \
        -Xclang -load -Xclang ${ANGORA_PATH}/pass/closure.so \
        -pie -fpic -g -O3 \
        -c -o angora_main_fuzz.o ./angora_main_fuzz.c 

ld -r \
        angora_main_fuzz.o \
        ${ANGORA_PATH}/lib/libclosure.a \
        -o angora_main_fuzz_merged.o

ar rcs angora_main_fuzz.a angora_main_fuzz_merged.o

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

/clang+llvm/bin/clang \
    ${PASS_FLAGS} \
    ${target}.c  -c -o ${target}.o

/clang+llvm/bin/clang \
    ${target}.o \
    ${LINK_FLAGS} \
    -o ${target}.fast
