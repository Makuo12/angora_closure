#!/bin/bash
ANGORA_PATH="$(pwd)"
echo "Building xpdf track binary..."
cd xpdf-4.06_2

mkdir -p build_track && cd build_track

USE_TRACK=1 cmake \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_C_COMPILER=${ANGORA_PATH}/bin/angora-clang \
        -DCMAKE_CXX_COMPILER=${ANGORA_PATH}/bin/angora-clang++ \
        -DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY \
        ..

USE_TRACK=1 make pdftotext -j$(nproc)
cd ../..
mkdir -p build_track
cp ./xpdf-4.06_2/build_track/xpdf/pdftotext build_track/pdftotext.taint