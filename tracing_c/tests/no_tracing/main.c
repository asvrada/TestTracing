#include <stdio.h>
#include <assert.h>

#include "rust_lib.h"

int main() {
    int result = addition(1, 2);

    assert(result == 3);

    return 0;
}
