set(log_file ${CMAKE_CURRENT_BINARY_DIR}/log.txt)
add_custom_target(remove_log_file
  COMMAND ${CMAKE_COMMAND} -E remove -f "${log_file}"
)

# This test will be run after tests/file_tracing since we defined the dependency.
# cmake -E cat <file> will fail if the file doesn't exist.
add_test(NAME tests/file_tracing-check_log_exists COMMAND ${CMAKE_COMMAND} -E cat ${log_file})
set_tests_properties(tests/file_tracing-check_log_exists PROPERTIES DEPENDS tests/file_tracing)

# Build test binary
add_executable(file_tracing main.c)

target_link_libraries(file_tracing ${RUST_LIB})
add_dependencies(file_tracing remove_log_file)

add_test(NAME tests/file_tracing COMMAND file_tracing)
