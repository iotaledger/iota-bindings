cmake_minimum_required(VERSION 3.1.0)

project(iota-bindings-tests)
enable_language(C CXX)

include(CTest)
include(ExternalProject)

# Global definitions
set(CMAKE_PROJECT_NAME "iota-bindings-tests")
set(CMAKE_MODULE_PATH
  ${CMAKE_SOURCE_DIR}/CMake/Modules
  )
set(CMAKE_C_FLAGS -Wall)

# Generic output
message(STATUS "Host processor is ${CMAKE_HOST_SYSTEM_PROCESSOR}")
message(STATUS "Current build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "Current build type flags: ${CMAKE_CXX_FLAGS_${CMAKE_BUILD_TYPE}}")

# Add rust library target
ExternalProject_Add(
    iota_static 
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --release
    COMMAND cargo build --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}/../native-static"
    INSTALL_COMMAND ""
    BUILD_ALWAYS ON
    LOG_BUILD ON)


# Includes
include_directories(
  ${CMAKE_SOURCE_DIR}/include/
  ${CMAKE_SOURCE_DIR}/../include/
  )

link_directories(
  ${CMAKE_SOURCE_DIR}/../native-static/target/release
  )

# Library source
file(GLOB_RECURSE TESTS_SRC src/*.cc src/include_c.c)

include(SetupGTest)
enable_testing()

add_executable(runUnitTests ${TESTS_SRC})
add_dependencies(runUnitTests iota_static)
target_link_libraries(runUnitTests googletest pthread m dl IOTA) 
target_compile_features(runUnitTests PRIVATE cxx_range_for)

add_test(AllTests runUnitTests)
