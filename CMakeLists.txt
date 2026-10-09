cmake_minimum_required(VERSION 3.10)
project(drop)

set(CMAKE_EXPORT_COMPILE_COMMANDS YES)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang" OR CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
	add_compile_options("-Wall" "-Wextra" "-Wpedantic" "-nostdlib++" "-nostdinc++")
endif()
if(LINUX AND CMAKE_BUILD_TYPE STREQUAL "Debug")
	add_compile_options("-O0")
	add_compile_options("-fsanitize=address")
	add_link_options("-fsanitize=address")
endif()

#region lib

file(GLOB_RECURSE LIB_SOURCES CONFIGURE_DEPENDS "lib/*.cpp")

add_library(libdrop_static STATIC ${LIB_SOURCES})
target_include_directories(libdrop_static PUBLIC "lib")
set_target_properties(libdrop_static PROPERTIES OUTPUT_NAME "drop")

add_library(libdrop SHARED ${LIB_SOURCES})
target_include_directories(libdrop PUBLIC "lib")
set_target_properties(libdrop PROPERTIES OUTPUT_NAME "drop")

#endregion

#region cli

file(GLOB_RECURSE CLI_SOURCES CONFIGURE_DEPENDS "cli/*.cpp")

add_executable(drop ${CLI_SOURCES})
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
	target_link_libraries(drop libdrop_static)
else()
	target_link_libraries(drop libdrop)
endif()

#endregion

#region tests

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
	file(GLOB_RECURSE TEST_SOURCES CONFIGURE_DEPENDS "cpp_tests/*.cpp")

	set(i 0)
	foreach(file ${TEST_SOURCES})
		if(NOT (file MATCHES "test_"))
			continue()
		endif()

		add_executable            ("test_${i}" "cpp_tests/testrt.cpp" ${file})
		target_include_directories("test_${i}" PRIVATE "cpp_tests")
		target_compile_definitions("test_${i}" PRIVATE "DROPTEST=${i}")
		target_link_libraries     ("test_${i}" libdrop_static)

		math(EXPR i "${i} + 1")
	endforeach()
endif()

#endregion
