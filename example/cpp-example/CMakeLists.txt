cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQURIED ON)
if (APPLE)
  set(CMAKE_CXX_FLAGS "-framework Security -framework CoreFoundation")
endif()
if (UNIX AND NOT APPLE)
  set(CMAKE_CXX_FLAGS "-lpthread -lssl -lcrypto -ldl")
endif()

project(cppexample VERSION 1.0)

add_subdirectory(sdk)

# static cppexample
add_executable(cppexamplestatic main.cc chainmain.cc cronos.cc)
target_link_libraries(cppexamplestatic PUBLIC ${DEFI_WALLET_CORE_CPP_LIB})

# dynamic cppexample
add_executable(cppexample main.cc chainmain.cc cronos.cc)
target_link_libraries(cppexample PUBLIC ${RUST_LIB} ${DEFI_WALLET_CORE_CPP_DYLIB} defi_wallet_core_cpp)
