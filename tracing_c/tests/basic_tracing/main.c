#include <stdio.h>
#include <assert.h>

#include "rust_lib.h"

int main() {
    init_tracing_basic();

    int result = addition(4, 5);
    assert(result == 9);

    return 0;
}
