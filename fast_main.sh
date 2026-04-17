#!/bin/bash
ANGORA_PATH="$(pwd)"
echo "Building xpdf track binary..."
cd xpdf-4.06_2
mkdir -p build_main && cd build_main

USE_FAST=1 cmake \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_C_COMPILER=${ANGORA_PATH}/bin/angora-clang \
        -DCMAKE_CXX_COMPILER=${ANGORA_PATH}/bin/angora-clang++ \
        -DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY \
        ..

USE_FAST=1 make pdftotext -j$(nproc)
cd ../..
mkdir -p build_main
cp ./xpdf-4.06_2/build_main/xpdf/pdftotext build_main/pdftotext.fast