
add_executable(no_tracing main.c)

# Add the compiled Rust library
target_link_libraries(no_tracing ${RUST_LIB})

add_test(NAME tests/no_tracing COMMAND no_tracing)
