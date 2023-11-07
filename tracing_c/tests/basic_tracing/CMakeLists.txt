
add_executable(basic_tracing main.c)

# Add the compiled Rust library
target_link_libraries(basic_tracing ${RUST_LIB})

add_test(NAME tests/basic_tracing COMMAND basic_tracing)
