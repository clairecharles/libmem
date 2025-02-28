cmake_minimum_required(VERSION 3.24)

include(ExternalProject)

project(libmem)

message(STATUS "[*] Platform: ${CMAKE_SYSTEM_NAME}")

option(LIBMEM_BUILD_TESTS "Build tests" ON) # builds the optional libmem test libraries.
option(LIBMEM_BUILD_STATIC "Build a static library" OFF) # builds libmem as a static library.

set(CAPSTONE_DIR "${PROJECT_SOURCE_DIR}/capstone")
set(CAPSTONE_INC "${CAPSTONE_DIR}/include")
set(CAPSTONE_IMPORT_DIR "${PROJECT_BINARY_DIR}/capstone-engine-prefix/src/capstone-engine-build")
ExternalProject_Add(capstone-engine SOURCE_DIR ${CAPSTONE_DIR} INSTALL_COMMAND echo CMAKE_ARGS -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE} -DCAPSTONE_ARCHITECTURE_DEFAULT=ON -DCAPSTONE_BUILD_TESTS=OFF -DCAPSTONE_BUILD_CSTOOL=OFF -DCAPSTONE_INSTALL=OFF -DCAPSTONE_BUILD_SHARED=OFF -DCMAKE_POSITION_INDEPENDENT_CODE=ON -DCAPSTONE_ARCHITECTURE_DEFAULT=OFF -DCAPSTONE_X86_SUPPORT=ON -DCAPSTONE_ARM_SUPPORT=ON -DCAPSTONE_ARM64_SUPPORT=ON)
add_library(capstone STATIC IMPORTED)
set_target_properties(capstone PROPERTIES IMPORTED_LOCATION ${CAPSTONE_IMPORT_DIR}/libcapstone.a)

set(KEYSTONE_DIR "${PROJECT_SOURCE_DIR}/keystone")
set(KEYSTONE_INC "${KEYSTONE_DIR}/include")
set(KEYSTONE_IMPORT_DIR "${PROJECT_BINARY_DIR}/keystone-engine-prefix/src/keystone-engine-build/llvm/lib")
ExternalProject_Add(keystone-engine SOURCE_DIR ${KEYSTONE_DIR} INSTALL_COMMAND echo CMAKE_ARGS -DBUILD_LIBS_ONLY=1 -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE} -DCMAKE_POSITION_INDEPENDENT_CODE=ON -DLLVM_TARGETS_TO_BUILD=X86;ARM;Aarch64)
add_library(keystone STATIC IMPORTED)
set_target_properties(keystone PROPERTIES IMPORTED_LOCATION ${KEYSTONE_IMPORT_DIR}/libkeystone.a)

set(LIEF_DIR "${PROJECT_SOURCE_DIR}/LIEF")
set(LIEF_INC "${LIEF_DIR}/include" "${PROJECT_BINARY_DIR}/lief-project-prefix/src/lief-project-build/include")
set(LIEF_IMPORT_DIR "${PROJECT_BINARY_DIR}/lief-project-prefix/src/lief-project-build")
set(LIEF_CMAKE_ARGS -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE} -DLIEF_USE_CCACHE=OFF -DLIEF_ENABLE_JSON=OFF -DLIEF_FROZEN_ENABLED=OFF -DLIEF_MACHO=OFF -DLIEF_OAT=OFF -DLIEF_DEX=OFF -DLIEF_VDEX=OFF -DLIEF_ART=OFF -DLIEF_EXTERNAL_LEAF=OFF -DLIEF_EXTERNAL_SPAN_DIR=OFF -DLIEF_FUZZING=OFF -DLIEF_PROFILING=OFF -DLIEF_COVERAGE=OFF -DLIEF_EXAMPLES=OFF -DLIEF_TESTS=OFF -DLIEF_DOC=OFF)
if(${CMAKE_SYSTEM_NAME} MATCHES Windows)
        set(LIEF_CMAKE_ARGS ${LIEF_CMAKE_ARGS} -DLIEF_ELF=OFF)
else()
        set(LIEF_CMAKE_ARGS ${LIEF_CMAKE_ARGS} -DLIEF_PE=OFF)
endif()
ExternalProject_Add(lief-project SOURCE_DIR ${LIEF_DIR} INSTALL_COMMAND echo CMAKE_ARGS ${LIEF_CMAKE_ARGS})
add_library(LIEF STATIC IMPORTED)
set_target_properties(LIEF PROPERTIES IMPORTED_LOCATION ${LIEF_IMPORT_DIR}/libLIEF.a)

set(LIBMEM_DIR "${PROJECT_SOURCE_DIR}/libmem")
set(LIBMEM_INC "${LIBMEM_DIR}/include")
file(GLOB_RECURSE LIBMEM_SRC "${LIBMEM_DIR}/src/*.c" "${LIBMEM_DIR}/src/*.cpp")
set(LIBMEM_DEPS capstone keystone LIEF stdc++ m)

if(NOT MSVC)
	add_compile_options(-Wall -Wextra -Wpedantic)
	if ("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
		add_compile_options(-g -ggdb)
	endif()
endif()

if (LIBMEM_BUILD_STATIC)
	add_library(libmem STATIC ${LIBMEM_SRC})
	add_custom_command(
		TARGET libmem POST_BUILD
		WORKING_DIRECTORY ${PROJECT_BINARY_DIR}
		COMMAND sh ${PROJECT_SOURCE_DIR}/makebundle.sh
	)
else()
	add_library(libmem SHARED ${LIBMEM_SRC})
endif()

include_directories(${PROJECT_SOURCE_DIR} ${LIBMEM_INC} ${CAPSTONE_INC} ${KEYSTONE_INC} ${LIEF_INC})
set_target_properties(libmem PROPERTIES PREFIX "" POSITION_INDEPENDENT_CODE True INCLUDES ${LIBMEM_INC})
target_compile_definitions(libmem PUBLIC LM_EXPORT)
add_dependencies(libmem capstone-engine keystone-engine lief-project)

if (LIBMEM_BUILD_TESTS)
	set(TESTS_DIR "${PROJECT_SOURCE_DIR}/tests")
	set(TEST1_SRC "${TESTS_DIR}/test1.c")
	set(TEST2_SRC "${TESTS_DIR}/test2.c")
	set(LIBTEST_SRC "${TESTS_DIR}/libtest.c")
	set(LIBTEST_PATH "${PROJECT_BINARY_DIR}/libtest${CMAKE_SHARED_LIBRARY_SUFFIX}")

	add_executable(test1 ${TEST1_SRC})
	target_include_directories(test1 PUBLIC ${LIBMEM_INC})
	target_link_libraries(test1 libmem)

	add_library(libtest SHARED ${LIBTEST_SRC})
	target_include_directories(libtest PUBLIC ${LIBMEM_INC})
	set_target_properties(libtest PROPERTIES PREFIX "")
	target_compile_definitions(test1 PUBLIC LIBTEST_PATH="${LIBTEST_PATH}")

	add_executable(test2 ${TEST2_SRC})
	target_include_directories(test2 PUBLIC ${LIBMEM_INC})
	target_link_libraries(test2 libmem)
	target_compile_definitions(test2 PUBLIC LIBTEST_PATH="${LIBTEST_PATH}" TEST1_NAME="test1${CMAKE_EXECUTABLE_SUFFIX}")

endif()

if(${CMAKE_SYSTEM_NAME} MATCHES Windows)
	set(LIBMEM_DEPS ${LIBMEM_DEPS} user32 psapi)

elseif(${CMAKE_SYSTEM_NAME} MATCHES Linux OR ${CMAKE_SYSTEM_NAME} MATCHES Android)
	set(LIBMEM_DEPS ${LIBMEM_DEPS} dl)
elseif(${CMAKE_SYSTEM_NAME} MATCHES FreeBSD)
	set(LIBMEM_DEPS ${LIBMEM_DEPS} dl kvm procstat elf)
else()
	message(FATAL_ERROR "[!] Unsupported platform")
endif()

target_link_libraries(libmem ${LIBMEM_DEPS})

install(TARGETS libmem
	LIBRARY DESTINATION lib)

install(DIRECTORY ${LIBMEM_INC}/libmem DESTINATION include)
