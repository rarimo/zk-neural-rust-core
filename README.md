# zk-neural-rust-core

This project provides a Rust core library for zero-knowledge proof generation and neural network inference, with C FFI bindings for integration with other languages/platforms.

## Structure

- `src/`
  - `core/` - Core logic for ZK proof, tensor operations, error handling, and callbacks.
  - `ffi.rs` - C FFI bindings for the Rust core.
  - `lib.rs` - Library entry point.
- `headers/zk_neural_rust_core.h` - C header generated for FFI.
- `test.c` - Example C usage/testing.
- `scripts/` - Build, test, and packaging scripts.

## Features

- Generate witnesses and proofs via user-provided callbacks.
- TensorFlow Lite model inference via `TensorInvoker`.
- C FFI for all major operations.
- Designed for cross-platform use, including iOS (see scripts).

## Usage

### Rust

Add as a dependency or use as a library crate.

### C FFI

1. Build the Rust library as a shared or static library.
2. Use the generated `zk_neural_rust_core.h` header.
3. See `test.c` for example usage.

### Scripts

- `scripts/build_ios.sh` - Build for iOS targets.
- `scripts/create_xcframework.sh` - Package as an XCFramework for Apple platforms.
- `scripts/create_tflitec_xcframework.sh` - Package TensorFlow Lite C dylib as an XCFramework.
- `scripts/run_test.sh` - Build and run the C test.
- `scripts/update_header.sh` - Update the C header file from Rust definitions.

## Example

See `test.c` for a demonstration of witness and proof generation via FFI.

## License

MIT License. See [LICENCE](LICENCE) for details.
