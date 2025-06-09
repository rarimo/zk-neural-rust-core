#!/bin/bash

set -e

XCFWNAME="ZkNeuralRustCore"
FWNAME="ZkNeuralRustCoreLib"

function create_framework() {
    for fw in "$@"; do
        copy_framework_files "${fw}"
    done

    local fw_paths=()
    for fw in "$@"; do
        fw_paths+=("-framework" "Frameworks/${fw}/$FWNAME.framework")
    done

    for fw in "$@"; do
        {
        echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        echo "<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">"
        echo "<plist version=\"1.0\">"
        echo "<dict>"
        echo "    <key>CFBundlePackageType</key>"
        echo "    <string>FMWK</string>"
        echo "    <key>CFBundleIdentifier</key>"
        echo "    <string>Rarilabs.$FWNAME</string>"
        echo "    <key>CFBundleExecutable</key>"
        echo "    <string>$FWNAME</string>"
        echo "    <key>CFBundleShortVersionString</key>"
        echo "    <string>1.0.0</string>"
        echo "    <key>CFBundleVersion</key>"
        echo "    <string>3</string>"
        echo "    <key>MinimumOSVersion</key>"
        echo "    <string>100</string>"
        echo "</dict>"
        echo "</plist>"
        } > "Frameworks/$fw/$FWNAME.framework/Info.plist"
    done

    rm -rf "Frameworks/$XCFWNAME.xcframework"
    xcrun xcodebuild -create-xcframework \
        "${fw_paths[@]}" \
        -output "Frameworks/$XCFWNAME.xcframework"

    
    if [ -n "$CODE_SIGNER" ]; then
        codesign --timestamp -s "$CODE_SIGNER" "Frameworks/$XCFWNAME.xcframework"
    fi
}

function copy_framework_files() {
    local FRAMEWORK_PATH="Frameworks/$1/ZkNeuralRustCoreLib.framework"

    mkdir -p "$FRAMEWORK_PATH/Headers"

    cp headers/zk_neural_rust_core.h "$FRAMEWORK_PATH/Headers/ZkNeuralRustCoreLib.h"

    mkdir -p $FRAMEWORK_PATH/Modules
    {
    echo "framework module ZkNeuralRustCoreLib {"
    echo "    umbrella header \"ZkNeuralRustCoreLib.h\""
    echo "    export *"
    echo "    module * { export * }"
    echo "}"
    } > $FRAMEWORK_PATH/Modules/module.modulemap

    cp target/$1/release/libzk_neural_rust_core.a $FRAMEWORK_PATH/ZkNeuralRustCoreLib
}

rm -rf Frameworks

strip -x target/aarch64-apple-ios/release/libzk_neural_rust_core.a target/aarch64-apple-ios-sim/release/libzk_neural_rust_core.a

frameworks=("aarch64-apple-ios" "aarch64-apple-ios-sim")
create_framework "${frameworks[@]}"

pushd "Frameworks"
zip -X -9 -r "$XCFWNAME.xcframework.zip" "$XCFWNAME.xcframework" -i */ZkNeuralRustCoreLib -i *.plist -i *.h -i *.modulemap
popd

swift package compute-checksum "Frameworks/ZkNeuralRustCore.xcframework.zip" > Frameworks/ZkNeuralRustCore.xcframework.zip.checksum

echo "ZkNeuralRustCore.xcframework.zip checksum: $(cat Frameworks/ZkNeuralRustCore.xcframework.zip.checksum)"
