cmake_minimum_required(VERSION 3.4)
project(angora_llvm_mode VERSION 1.3.0 LANGUAGES C CXX ASM)

# 1. Properly find LLVM
find_package(LLVM REQUIRED CONFIG)

# 2. Add LLVM's CMake module path so macros like 'llvm_map_components_to_libnames' work
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
include(AddLLVM)

add_definitions(${LLVM_DEFINITIONS})
include_directories(${LLVM_INCLUDE_DIRS})
link_directories(${LLVM_LIBRARY_DIRS})

# Standard Angora includes
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../runtime/include)
include_directories(include)
include_directories(dfsan_rt)

set(ANGORA_BIN_DIR ".")
set(ANGORA_LIB_DIR "lib")
set(ANGORA_RULE_DIR "rules")
set(ANGORA_PASS_DIR "pass")

# Subdirectories
add_subdirectory(compiler)
add_subdirectory(pass)
add_subdirectory(closure)
add_subdirectory(rules)
add_subdirectory(libcxx)
add_subdirectory(external_lib)
add_subdirectory(dfsan_rt)


# add_custom_target(run_script ALL
#     COMMAND chmod +x ${CMAKE_CURRENT_SOURCE_DIR}/track_shell.sh
#     COMMAND ${CMAKE_CURRENT_SOURCE_DIR}/track_shell.sh
#     DEPENDS pass closure rules libcxx external_lib dfsan_rt
#     WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/..
#     COMMENT "Running build script..."
# )