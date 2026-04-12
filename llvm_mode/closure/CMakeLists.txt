# No need to repeat cmake_minimum_required if already in parent, 
# but keeping it for safety if building standalone.
# cmake_minimum_required(VERSION 3.10)
# project(closure)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT LLVM_FOUND)
    find_package(LLVM REQUIRED CONFIG)
    list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
    include(AddLLVM)
endif()

# remove this — not needed since we are not linking LLVM
# llvm_map_components_to_libnames(llvm_libs core support irreader)

add_library(closure MODULE
    Common.cpp
    RenameMainPass.cpp
    HeapResetPass.cpp
    ExitHookPass.cpp
    CloneGlobalsPass.cpp
    FileHookPass.cpp
    Pass.cpp
)

set_target_properties(closure PROPERTIES
    PREFIX ""
    COMPILE_FLAGS "-fno-rtti -fPIC"
)

# remove this line — LLVM symbols come from clang at runtime
# target_link_libraries(closure PRIVATE ${llvm_libs})

install(TARGETS closure DESTINATION ${ANGORA_PASS_DIR})