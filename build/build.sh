#!/bin/bash
BIN_PATH=$(readlink -f "$0")
ROOT_DIR=$(dirname $(dirname $BIN_PATH))

set -euxo pipefail

if ! [ -x "$(command -v llvm-config)"  ]; then
    ${ROOT_DIR}/build/install_llvm.sh
    export PATH=${HOME}/clang+llvm/bin:$PATH
    export LD_LIBRARY_PATH=${HOME}/clang+llvm/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
    export CC=clang
    export CXX=clang++
fi

PREFIX=${PREFIX:-${ROOT_DIR}/bin/}

cargo build
cargo build --release

rm -rf ${PREFIX}
mkdir -p ${PREFIX}
mkdir -p ${PREFIX}/lib
cp target/release/*.a ${PREFIX}/lib

if [ "${FUZZER_TYPE}" = "angora" ]; then
    cd llvm_mode
    rm -rf build
    mkdir -p build
    cd build
    cmake -DCMAKE_INSTALL_PREFIX=${PREFIX} -DCMAKE_BUILD_TYPE=Release ..
    make -j # VERBOSE=1
    make install # VERBOSE=1
else 
    cd fuzzer_normal
    rm -rf build
    mkdir -p build
    cd build
    cmake -DCMAKE_INSTALL_PREFIX=${PREFIX} -DCMAKE_BUILD_TYPE=Release ..
    make -j # VERBOSE=1
    make install # VERBOSE=1
    # Go back to the root or use the absolute variable
    # This ensures we find the script regardless of where we are
    chmod +x "${ROOT_DIR}/normal_shell.sh"
    "${ROOT_DIR}/normal_shell.sh"
fi


