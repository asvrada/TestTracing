#include <stdio.h>
#include <assert.h>

#include "rust_lib.h"

int main() {
    init_tracing_file();

    int result = addition(6, 7);
    assert(result == 13);

    return 0;
}
