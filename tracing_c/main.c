#include <stdio.h>

#include "rust_lib.h"

int main(int argc, char **argv)
{
    init_tracing_basic();

    int result = addition(1, 2);

    return 0;
}
