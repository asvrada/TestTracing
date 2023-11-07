all: build

build:
	make build_rust
	make build_c

build_rust:
	cd tracing_rust && cargo build --release

config_c: build_rust
	cd tracing_c && mkdir -p build && cd build && cmake ..

build_c: config_c
	cd tracing_c/build && cmake --build . -j

# RUST_LOG=trace make ctest
ctest: build_c
	cd tracing_c/build && ctest -V

clean:
	cd tracing_rust && cargo clean
	rm -rf tracing_c/build
