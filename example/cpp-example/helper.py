#!/usr/bin/env python3
import fnmatch
import os
import shutil

SOURCES = [
    "main.cc",
    "cronos.cc",
]
CPP_EXAMPLE_PATH = "."
VS_EXAMPLE_PATH = "../vs-example/vs-example"

INITIAL_INCLUDE = '#include "defi-wallet-core-cpp/src/lib.rs.h"'
FINAL_INCLUDE = '#include "lib.rs.h"'
TARGET_DIR = "../../target/release"

OUT_DIR = "../../target/cxxbridge"


def copy_to(output_path):
    copy_cxxbridge(output_path)
    copy_lib_files(output_path)


# copy the generated binding files: `*.cc` and `*.h` to `output_path`
def copy_cxxbridge(output_path):
    files = []
    files.extend(collect_files("*.h", OUT_DIR))
    files.extend(collect_files("*.cc", OUT_DIR))

    # replace string
    for filename in files:
        # Safely read the input filename using 'with'
        with open(filename) as f:
            s = f.read()
            if INITIAL_INCLUDE not in s:
                continue

        # Safely write the changed content, if found in the file
        with open(filename, "w") as f:
            s = s.replace(INITIAL_INCLUDE, FINAL_INCLUDE)
            f.write(s)

    # copy the bindings, need python 3.8+
    shutil.copytree(OUT_DIR, output_path, dirs_exist_ok=True)


# copy library files: `*.a`, `*.dylib`, and `*.dll.lib` (windows) to `output_path`
def copy_lib_files(output_path):
    files = []
    # workaround: concat the library name manually
    files.extend(collect_files("*.a", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.dylib", TARGET_DIR, recursive=False))
    files.extend(collect_files("*.dll.lib", TARGET_DIR, recursive=False))
    # workaround: search libcxxbridge1.a and push the first one
    files.append(collect_files("libcxxbridge1.a", TARGET_DIR)[0])

    # copy the libraries, need python 3.8+
    for f in files:
        shutil.copy(f, output_path)


# copy `SOURCES` to `output_path`
def copy_example_files(output_path):
    for f in SOURCES:
        shutil.copy(f, output_path)


# collect files with `pattern` in `path`, and return the matched files
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
    copy_to(CPP_EXAMPLE_PATH)
    copy_to(VS_EXAMPLE_PATH)
    copy_example_files(VS_EXAMPLE_PATH)
