#!/usr/bin/env python3
import fnmatch
import os
import shutil
from pathlib import Path

EXAMPLE_SOURCES = [
    "main.cc",
    "chainmain.cc",
    "cronos.cc",
    "chainmain.h",
    "cronos.h",
    "wallet.cc",
    "wallet.h",
]

SOURCES = [
    "../../bindings/cpp/src/nft.cc",
    "../../bindings/cpp/include/nft.h",
]

CPP_EXAMPLE_PATH = Path(__file__).parent
VS_EXAMPLE_PATH = Path(__file__).parent.parent / "vs-example/vs-example"

INCLUDE_PATH = "sdk/include"
LIB_PATH = "sdk/lib"

INITIAL_INCLUDES = [
    '#include "defi-wallet-core-cpp/src/lib.rs.h"',
    '#include "defi-wallet-core-cpp/src/uint.rs.h"',
    '#include "defi-wallet-core-cpp/include/nft.h"',
]

FINAL_INCLUDES = [
    '#include "lib.rs.h"',
    '#include "uint.rs.h"',
    '#include "../../nft.h"',
]

INITIAL_SOURCES_INCLUDES = [
    '#include "defi-wallet-core-cpp/include/nft.h"',
]
FINAL_SOURCES_INCLUDES = ['#include "nft.h"']


# the path of output target, defined by --target_dir
TARGET_DIR = None

# the path of cxxbridge artifacts
OUT_DIR = None


# copy the generated binding files: `*.cc` and `*.h` to `output_path`
def copy_cxxbridge(output_path):
    files = []
    files.extend(collect_files("*.h", OUT_DIR))
    files.extend(collect_files("*.cc", OUT_DIR))

    def has_include_string(s):
        for include in INITIAL_INCLUDES:
            if include in s:
                return True
        return False

    # replace string
    for filename in files:
        # Safely read the input filename using 'with'
        with open(filename) as f:
            s = f.read()
            if not has_include_string(s):
                continue

        # Safely write the changed content, if found in the file
        with open(filename, "w") as f:
            for i, include in enumerate(INITIAL_INCLUDES):
                s = s.replace(include, FINAL_INCLUDES[i])
            f.write(s)

    # copy the bindings, need python 3.8+
    shutil.copytree(OUT_DIR, output_path, dirs_exist_ok=True)
    print("Copied", OUT_DIR, "to", output_path)


# copy library files: `*.a`, `*.dylib`, `*.lib` (windows), `*.dll` (windows), `*.so`
# (linux) to `output_path`
def copy_lib_files(output_path):
    os.makedirs(output_path, exist_ok=True)
    files = []
    files.extend(collect_files("*.a", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.dylib", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.lib", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.dll", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.so", TARGET_DIR, recursive=False))
    # workaround: search libcxxbridge1.a and push the first one
    files.append(collect_files("libcxxbridge1.a", TARGET_DIR)[0])

    # copy the libraries, need python 3.8+
    for f in files:
        shutil.copy(f, output_path)
        print("Copied", f, "to", output_path)


# copy `EXAMPLE_SOURCES` to `output_path`
def copy_example_files(output_path):
    for f in EXAMPLE_SOURCES:
        shutil.copy(f, output_path)


# copy `SOURCES` to `output_path`
def copy_sources_files(output_path):
    for f in SOURCES:
        shutil.copy(f, output_path)
    files = []
    files.extend(collect_files("*.h", output_path, recursive=False))
    files.extend(collect_files("*.cc", output_path, recursive=False))

    def has_include_string(s):
        for include in INITIAL_SOURCES_INCLUDES:
            if include in s:
                return True
        return False

    # replace string
    for filename in files:
        # Safely read the input filename using 'with'
        with open(filename) as f:
            s = f.read()
            if not has_include_string(s):
                continue

        # Safely write the changed content, if found in the file
        with open(filename, "w") as f:
            for i, include in enumerate(INITIAL_SOURCES_INCLUDES):
                s = s.replace(include, FINAL_SOURCES_INCLUDES[i])
            f.write(s)


# collect files with `pattern` in `search path`, and return the matched files
def collect_files(pattern, search_path, recursive=True):
    result = []
    if recursive:
        for root, dirs, files in os.walk(search_path):
            for name in files:
                if fnmatch.fnmatch(name, pattern):
                    result.append(os.path.join(root, name))
    else:
        for f in os.listdir(search_path):
            # if os.path.isfile(os.path.join(search_path, f)):
            if fnmatch.fnmatch(f, pattern):
                result.append(os.path.join(search_path, f))

    return result


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(
        description="Generate bindings for the C++ example."
    )
    parser.add_argument(
        "--target_dir", metavar="path", required=True, help="path to target dir"
    )
    args = parser.parse_args()
    TARGET_DIR = args.target_dir
    OUT_DIR = Path(TARGET_DIR).parent / "cxxbridge"
    print("TARGET_DIR= ", TARGET_DIR)
    print("OUT_DIR= ", OUT_DIR)
    copy_cxxbridge(CPP_EXAMPLE_PATH / INCLUDE_PATH)
    copy_lib_files(CPP_EXAMPLE_PATH / LIB_PATH)
    copy_sources_files(CPP_EXAMPLE_PATH / INCLUDE_PATH)

    copy_cxxbridge(VS_EXAMPLE_PATH / INCLUDE_PATH)
    copy_lib_files(VS_EXAMPLE_PATH / LIB_PATH)
    copy_sources_files(VS_EXAMPLE_PATH / INCLUDE_PATH)

    copy_example_files(VS_EXAMPLE_PATH)
