#!/bin/bash

set -e

if [ -d "xcframeworks" ]; then
    rm -rf xcframeworks
fi

if [ -z "$CODE_SIGNER" ]; then
    echo "CODE_SIGNER is not set. Please set it to your code signing identity. Use \"security find-identity -v -p codesigning\" to find your identity."
    exit 1
fi 


strip -x target/aarch64-apple-ios/release/libzk_neural_rust_core.a target/aarch64-apple-ios-sim/release/libzk_neural_rust_core.a

xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/libzk_neural_rust_core.a -headers headers \
    -library target/aarch64-apple-ios-sim/release/libzk_neural_rust_core.a -headers headers \
    -output xcframeworks/ZkNeuralRustCore.xcframework

codesign --timestamp -s "$CODE_SIGNER" xcframeworks/ZkNeuralRustCore.xcframework

zip -9 -r "xcframeworks/ZkNeuralRustCore.xcframework.zip" "xcframeworks/ZkNeuralRustCore.xcframework"

swift package compute-checksum "xcframeworks/ZkNeuralRustCore.xcframework.zip" > xcframeworks/ZkNeuralRustCore.xcframework.zip.checksum

echo "ZkNeuralRustCore.xcframework.zip checksum: $(cat xcframeworks/ZkNeuralRustCore.xcframework.zip.checksum)"
