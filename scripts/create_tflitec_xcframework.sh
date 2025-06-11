#!/bin/bash

set -e

rm -rf TensorFrameworks

IOS_FRAMEWORK_PATH=$(ls -d ./target/aarch64-apple-ios/release/build/tflitec-*/out/TensorFlowLiteC.framework | head -n 1)
IOS_SIM_FRAMEWORK_PATH="./build_helpers/ios-arm64-simulator/TensorFlowLiteC.framework"

mkdir -p TensorFrameworks

xcrun xcodebuild -create-xcframework \
    -framework "$IOS_FRAMEWORK_PATH" \
    -framework "$IOS_SIM_FRAMEWORK_PATH" \
    -output "TensorFrameworks/TensorFlowLiteC.xcframework"

pushd "TensorFrameworks"
zip -9 -r "TensorFlowLiteC.xcframework.zip" "TensorFlowLiteC.xcframework"
popd

swift package compute-checksum "TensorFrameworks/TensorFlowLiteC.xcframework.zip" > TensorFrameworks/TensorFlowLiteC.xcframework.zip.checksum

echo "TensorFlowLiteC.xcframework.zip checksum: $(cat TensorFrameworks/TensorFlowLiteC.xcframework.zip.checksum)"
