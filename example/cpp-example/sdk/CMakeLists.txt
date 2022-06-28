cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQURIED ON)

project(defi_wallet_core_cpp VERSION 0.0.1)

# Find bindings source files
file(GLOB_RECURSE DEFI_WALLET_CORE_CPP_BINDINGS include/defi-wallet-core-cpp/src/*.cc)
file(GLOB DEFI_WALLET_CORE_CPP_SROUCES include/*.cc)

# Find the rust types binding library
find_library(RUST_LIB libcxxbridge1.a REQUIRED PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)

# Find the prebuilt static and dynamic libraries
if (WIN32)
  find_library(DEFI_WALLET_CORE_CPP_LIB defi_wallet_core_cpp.lib PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
  find_library(DEFI_WALLET_CORE_CPP_DYLIB defi_wallet_core_cpp.dll.lib PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
endif()

if (APPLE)
  find_library(DEFI_WALLET_CORE_CPP_LIB libdefi_wallet_core_cpp.a PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
  find_library(DEFI_WALLET_CORE_CPP_DYLIB libdefi_wallet_core_cpp.dylib PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
endif()

if (UNIX AND NOT APPLE)
  find_library(DEFI_WALLET_CORE_CPP_LIB libdefi_wallet_core_cpp.a PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
  find_library(DEFI_WALLET_CORE_CPP_DYLIB libdefi_wallet_core_cpp.so PATHS ${CMAKE_CURRENT_SOURCE_DIR}/lib)
endif()

# Add library defi_wallet_core_cpp built from bindings source files
add_library(defi_wallet_core_cpp ${DEFI_WALLET_CORE_CPP_BINDINGS} ${DEFI_WALLET_CORE_CPP_SROUCES})
