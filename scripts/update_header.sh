#!/bin/bash

set -e

cbindgen --config cbindgen.toml --crate zk-neural-rust-core --output ./headers/zk_neural_rust_core.h
