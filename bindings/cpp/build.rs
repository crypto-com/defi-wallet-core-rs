use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

const SOURCES: &[&str] = &[
    "../../example/cpp-example/main.cc",
    "../../example/cpp-example/cronos.cc",
];
const BRIDGES: &[&str] = &["src/lib.rs", "src/nft.rs"];
const CPP_EXAMPLE_PATH: &str = "../../example/cpp-example";
const VS_EXAMPLE_PATH: &str = "../../example/vs-example/vs-example";

const INITIAL_INCLUDE: &str = "#include \"defi-wallet-core-cpp/src/lib.rs.h\"";
const FINAL_INCLUDE: &str = "#include \"lib.rs.h\"";

fn main() {
    cxx_build::bridges(BRIDGES)
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");

    for bridge in SOURCES {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    for bridge in BRIDGES {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    // this can make sure we run copy in every build, enable if necessary
    // println!("cargo:rerun-if-changed={}", CPP_EXAMPLE_PATH);
    // this can make sure we run copy in every build, enable if necessary
    // println!("cargo:rerun-if-changed={}", VS_EXAMPLE_PATH);

    copy_to(CPP_EXAMPLE_PATH);
    copy_to(VS_EXAMPLE_PATH);
    copy_example_files(VS_EXAMPLE_PATH)
}

fn copy_to(output_path: &str) {
    copy_source_output_files(output_path);
    copy_cxx_files(output_path);
    copy_lib_files(output_path);
}

/// copy the generated binding files: `*.cc` and `*.h` to `output_path`
fn copy_source_output_files(output_path: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let include_path = Path::new(&out_dir).join("cxxbridge/include");
    let source_path = Path::new(&out_dir).join("cxxbridge/sources");
    let example_path = Path::new(output_path);

    let mut files: Vec<PathBuf> = vec![];
    collect_files(&include_path, &mut files, "h");
    collect_files(&source_path, &mut files, "cc");
    // println!("cargo:warning={:?}",files);

    // workaround: replace the include string
    for f in files.clone() {
        if let Ok(contents) = fs::read_to_string(&f) {
            if contents.contains(INITIAL_INCLUDE) {
                let new_data = contents.replace(INITIAL_INCLUDE, FINAL_INCLUDE);
                if let Ok(mut dst) = fs::File::create(&f) {
                    if let Err(e) = dst.write(new_data.as_bytes()) {
                        println!("cargo:warning=write {:?} failed", e);
                    }
                } else {
                    println!("cargo:warning=create {:?} failed", f);
                }
            } else {
                println!("cargo:warning=read {:?} failed", f);
            }
        }
    }

    // clear and create binding source folder, then copy bindings files
    let source_output_path = example_path
        .join(env::var("CARGO_PKG_NAME").unwrap())
        .join("src");
    // println!(
    //     "cargo:warning=source output folder is {:?}",
    //     source_output_path
    // );
    if source_output_path.exists() {
        fs::remove_dir_all(source_output_path.clone()).unwrap();
    }
    fs::create_dir_all(source_output_path.clone()).unwrap();
    copy_files(files, &source_output_path);
}

/// copy cxx.h to `output_path`
fn copy_cxx_files(output_path: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let include_path = Path::new(&out_dir).join("cxxbridge/include");
    let example_path = Path::new(output_path);
    // clear and create cxx output folder, then copy cxx.h
    let cxx_output_path = example_path.join("rust");
    // println!("cargo:warning=cxx output folder is {:?}", cxx_output_path);
    if cxx_output_path.exists() {
        fs::remove_dir_all(cxx_output_path.clone()).unwrap();
    }
    fs::create_dir_all(cxx_output_path.clone()).unwrap();

    let mut files: Vec<PathBuf> = vec![];
    files.push(include_path.join("rust").join("cxx.h"));
    copy_files(files, &cxx_output_path);
}

/// copy library files: `*.a` and `*.dylib` to `output_path`
fn copy_lib_files(output_path: &str) {
    let example_path = Path::new(output_path);
    let mut files: Vec<PathBuf> = vec![];
    // println!("cargo:warning={:?}", env::var("CARGO_TARGET_DIR")); // Not working
    // workaround: concat the library name manually
    files.push(
        get_target_path()
            .join(format!("lib{}.a", env::var("CARGO_PKG_NAME").unwrap()).replace("-", "_")),
    );
    files.push(
        get_target_path()
            .join(format!("lib{}.dylib", env::var("CARGO_PKG_NAME").unwrap()).replace("-", "_")),
    );

    // workaround: search libcxxbridge1.a and push the first one
    let libcxxbridge1_files: Vec<PathBuf> = WalkDir::new(get_target_path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "libcxxbridge1.a")
        .map(|e| e.into_path())
        .collect();

    files.push(libcxxbridge1_files[0].clone());

    copy_files(files, example_path);
}

// copy `SOURCES` to `output_path`
fn copy_example_files(output_path: &str) {
    let mut files: Vec<PathBuf> = vec![];
    for f in SOURCES {
        files.push(Path::new(f).to_owned());
    }
    let output_path = Path::new(output_path);
    copy_files(files, output_path);
}

/// copy `files` to `dst`
fn copy_files(files: Vec<PathBuf>, dst: &Path) {
    for f in files {
        // println!("cargo:warning={:?}", f);
        match &f.file_name() {
            Some(filename) => {
                if let Err(e) = std::fs::copy(&f, &dst.join(filename)) {
                    println!("cargo:warning={:?}", e);
                }
            }
            None => {
                println!("cargo:warning=copy {:?} failed", f);
            }
        }
    }
}

/// collect files with `extension` in `path`, and save the file list to `files`
fn collect_files(path: &PathBuf, files: &mut Vec<PathBuf>, extension: &str) {
    files.append(
        &mut WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().is_some()
                    && e.path().extension().unwrap() == extension
            })
            .map(|e| e.into_path())
            .collect(),
    )
}

fn get_target_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("../../target")
        .join(profile);
    return PathBuf::from(path);
}
