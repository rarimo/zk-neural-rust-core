#!/bin/bash

set -e

cargo build --release
gcc -o test test.c -L./target/release -lzk_neural_rust_core -I.
export DYLD_LIBRARY_PATH=target/release  # macOS
./test
