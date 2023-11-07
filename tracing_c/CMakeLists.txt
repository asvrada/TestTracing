cmake_minimum_required(VERSION 3.18.0)
project(tracing_c VERSION 0.1.0 LANGUAGES C)

include(CTest)

# Add include folder globally
include_directories(include/)

# Find the compiled Rust library
find_library(RUST_LIB tracing_rust PATHS "${CMAKE_SOURCE_DIR}/../tracing_rust/target/release/")
if ("RUST_LIB-NOTFOUND" STREQUAL ${RUST_LIB})
    message(FATAL_ERROR "Cannot find compiled Rust lib!!!")
endif ()

add_executable(tracing_c main.c)

# Link the compiled Rust library
target_link_libraries(tracing_c PUBLIC ${RUST_LIB})

# Configure CPack
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

# Configure tests
if (BUILD_TESTING)
    add_subdirectory(tests)
endif ()
