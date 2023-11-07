# TestTracing
This repo experiments collecting tracing data from Rust in C.

## Tracing in Rust

To generate and collect tracing data, `tracing` and `tracing-subscriber` crates are used respectively.

For a typical Rust library, it should only use `tracing` to generate tracing data. It is its client (a Rust executable) that should properly configure `tracing-subscriber` to collect tracing data. However, if we want to collect tracing data from a C program, we need to configure `tracing-subscriber` in Rust, and provide means for C program to receive the data.

There are 2 means to do that, during Rust lib initialization:
1. (Preferred) accept a callback function from C, with a signature like
```C
void init_rust_lib(void (*log_callback)(int level, const char* content, ...))
```

This method provides the maximum freedom to C Program on how they want to handle the logs, they can print it to stdout, or stderr, or write to log file. But need to investigate how to make `tracing-subscriber` expose raw strings.

2. accept a `FILE` from C
```C
void init_rust_lib(FILE* log_file)
```

In the Rust library, if we can convert `FILE*` from C to something that `impl io::Write`, then we could configure `tracing-subscriber` like so

```Rust
tracing_subscriber::fmt()
    .with_env_filter(filter)
    .with_writer(log_file)
    .init();
```

But this is less flexible.

## To run
```bash
make ctest
# -- or --
RUST_LOG=trace make ctest
```
