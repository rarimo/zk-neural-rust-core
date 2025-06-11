#!/bin/bash

set -e

rm -rf TensorFrameworks

IOS_FRAMEWORK_PATH=$(ls -d ./target/aarch64-apple-ios/release/build/tflitec-*/out/TensorFlowLiteC.framework | head -n 1)
IOS_SIM_FRAMEWORK_PATH=$(ls -d ./target/aarch64-apple-ios-sim/release/build/tflitec-*/out/TensorFlowLiteC.framework | head -n 1)

function create_plist() {
    for fw in "$@"; do
        {
        echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        echo "<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">"
        echo "<plist version=\"1.0\">"
        echo "<dict>"
        echo "    <key>CFBundlePackageType</key>"
        echo "    <string>FMWK</string>"
        echo "    <key>CFBundleIdentifier</key>"
        echo "    <string>Rarilabs.TensorFlowLiteC</string>"
        echo "    <key>CFBundleExecutable</key>"
        echo "    <string>TensorFlowLiteC</string>"
        echo "    <key>CFBundleShortVersionString</key>"
        echo "    <string>1.0.0</string>"
        echo "    <key>CFBundleVersion</key>"
        echo "    <string>3</string>"
        echo "    <key>MinimumOSVersion</key>"
        echo "    <string>100</string>"
        echo "</dict>"
        echo "</plist>"
        } > "TensorFrameworks/TensorFlowLiteC.xcframework/$fw/TensorFlowLiteC.framework/Info.plist"
    done
}

xcrun xcodebuild -create-xcframework \
    -framework "$IOS_FRAMEWORK_PATH" \
    -framework "$IOS_SIM_FRAMEWORK_PATH" \
    -output "TensorFrameworks/TensorFlowLiteC.xcframework"

frameworks=("ios-arm64" "ios-arm64-simulator")
create_plist "${frameworks[@]}"

pushd "TensorFrameworks"
zip -9 -r "TensorFlowLiteC.xcframework.zip" "TensorFlowLiteC.xcframework"
popd

swift package compute-checksum "TensorFrameworks/TensorFlowLiteC.xcframework.zip" > TensorFrameworks/TensorFlowLiteC.xcframework.zip.checksum

echo "TensorFlowLiteC.xcframework.zip checksum: $(cat TensorFrameworks/TensorFlowLiteC.xcframework.zip.checksum)"
