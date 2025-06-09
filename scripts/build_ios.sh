#!/bin/bash

set -e

rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim

cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release
