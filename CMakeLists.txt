cmake_minimum_required(VERSION 3.10)
project(drop)

set(CMAKE_EXPORT_COMPILE_COMMANDS YES)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS "src/*.cpp")

if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang" OR CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
	add_compile_options("-Wall" "-Wextra" "-Wpedantic")
endif()

add_executable(drop ${SOURCES})
