#!/bin/bash

OS=`uname | tr 'A-Z' 'a-z'`
if [ "$OS" != "darwin" -a  "$OS" != "linux" ]
then
    echo "not support for $OS"
    exit 1
fi

BINARYENVERSION="105"

if [ "$OS" == "linux" ] && [ ! -f target/binaryen-version_$BINARYENVERSION/bin/wasm2js ]
then
    rm -f target/binaryen-version_$BINARYENVERSION-x86_64-linux.tar.gz
    wget https://github.com/WebAssembly/binaryen/releases/download/version_$BINARYENVERSION/binaryen-version_$BINARYENVERSION-x86_64-linux.tar.gz -P target/
    tar zxf target/binaryen-version_$BINARYENVERSION-x86_64-linux.tar.gz -C target/
    rm -f target/binaryen-version_$BINARYENVERSION-x86_64-linux.tar.gz
fi

if [ "$OS" == "darwin" ] && [ ! -f target/binaryen-version_$BINARYENVERSION/bin/wasm2js ]
then
    CPU=`uname -a | awk '{print $NF}'`
    if [ "$CPU" == "x86_64" ]
    then
        rm -f target/binaryen-version_$BINARYENVERSION-x86_64-macos.tar.gz
        wget https://github.com/WebAssembly/binaryen/releases/download/version_$BINARYENVERSION/binaryen-version_$BINARYENVERSION-x86_64-macos.tar.gz -P target/
        tar zxf target/binaryen-version_$BINARYENVERSION-x86_64-macos.tar.gz -C target/
        rm -f target/binaryen-version_$BINARYENVERSION-x86_64-macos.tar.gz
    elif [ "$CPU" == "arm64" ] && [ ! -f target/binaryen-version_$BINARYENVERSION/bin/wasm2js ]
    then
        rm -f target/binaryen-version_$BINARYENVERSION-arm64-macos.tar.gz
        wget https://github.com/WebAssembly/binaryen/releases/download/version_$BINARYENVERSION/binaryen-version_$BINARYENVERSION-arm64-macos.tar.gz -P target/
        tar zxf target/binaryen-version_$BINARYENVERSION-arm64-macos.tar.gz -C target/
        rm -f target/binaryen-version_$BINARYENVERSION-arm64-macos.tar.gz
    else
        echo "not support for $CPU"
        exit 1
    fi
fi

target/binaryen-version_$BINARYENVERSION/bin/wasm2js bindings/wasm/pkg/defi_wallet_core_wasm_bg.wasm -o bindings/wasm/pkg/defi_wallet_core_wasm_bg.wasm.js

if [ "$OS" == "linux" ];then
    sed -i 's/defi_wallet_core_wasm_bg.wasm/defi_wallet_core_wasm_bg.wasm.js/' bindings/wasm/pkg/defi_wallet_core_wasm_bg.js
    sed -i 's/defi_wallet_core_wasm_bg.wasm/defi_wallet_core_wasm_bg.wasm.js/' bindings/wasm/pkg/defi_wallet_core_wasm.js
    sed -i '5i\    "defi_wallet_core_wasm_bg.wasm.js",' bindings/wasm/pkg/package.json
    sed -i '5i\    "snippets",' bindings/wasm/pkg/package.json
elif [ "$OS" == "darwin" ];then
    sed -i '' -e $'s/defi_wallet_core_wasm_bg.wasm/defi_wallet_core_wasm_bg.wasm.js/' bindings/wasm/pkg/defi_wallet_core_wasm_bg.js
    sed -i '' -e $'s/defi_wallet_core_wasm_bg.wasm/defi_wallet_core_wasm_bg.wasm.js/' bindings/wasm/pkg/defi_wallet_core_wasm.js
    sed -i '' '5i\'$'\n''    "defi_wallet_core_wasm_bg.wasm.js",' bindings/wasm/pkg/package.json
    sed -i '' '5i\'$'\n''    "snippet",' bindings/wasm/pkg/package.json
else
    echo "not support for $OS"
    exit 1
fi

echo "export var __wbindgen_export_2 = retasmFunc.__wbindgen_export_2;" >> bindings/wasm/pkg/defi_wallet_core_wasm_bg.wasm.js

echo "finish"