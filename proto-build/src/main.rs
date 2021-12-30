// Copyright (c) 2020-2021, Cosmos Rust authors (licensed under the Apache License Version 2.0)
use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
// TODO(tarcieri): use a logger for this
static QUIET: AtomicBool = AtomicBool::new(false);

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.44.5";

/// The directory generated cosmos-sdk proto files go into in this repo
const COSMOS_SDK_PROTO_DIR: &str = "../defi-wallet-core-proto/src/prost/";

/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../third_party/cosmos-sdk";

const CHAIN_MAIN_REV: &str = "v3.3.3";
const CHAIN_MAIN_DIR: &str = "../third_party/chain-main";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];
/// Regex for locating instances of `tendermint-proto` in prost/tonic build output
const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
/// Attribute preceeding a Tonic client definition
const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];

/// Log info to the console (if `QUIET` is disabled)
// TODO(tarcieri): use a logger for this
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = COSMOS_SDK_PROTO_DIR.parse().unwrap();
    let _cosmos_sdk_dir: PathBuf = COSMOS_SDK_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    fs::create_dir(tmp_build_dir.clone()).unwrap();

    update_submodules();
    output_commit_versions(&tmp_build_dir);
    compile_sdk_protos_and_services(&tmp_build_dir);
    compile_chain_main_protos_and_services(&tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir);
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

fn update_submodules() {
    info!("Updating cosmos/cosmos-sdk submodule...");
    run_git(&["-C", COSMOS_SDK_DIR, "submodule", "update", "--init"]);
    run_git(&["-C", COSMOS_SDK_DIR, "fetch"]);
    run_git(&["-C", COSMOS_SDK_DIR, "reset", "--hard", COSMOS_SDK_REV]);

    info!("Updating chain-main submodule...");
    run_git(&["-C", CHAIN_MAIN_DIR, "submodule", "update", "--init"]);
    run_git(&["-C", CHAIN_MAIN_DIR, "fetch"]);
    run_git(&["-C", CHAIN_MAIN_DIR, "reset", "--hard", CHAIN_MAIN_REV]);
}

fn output_commit_versions(out_dir: &Path) {
    let path = out_dir.join("COSMOS_SDK_COMMIT");
    fs::write(path, COSMOS_SDK_REV).unwrap();

    let path = out_dir.join("CHAIN_MAIN_COMMIT");
    fs::write(path, CHAIN_MAIN_REV).unwrap();
}

fn compile_sdk_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");
    let sdk_dir = Path::new(COSMOS_SDK_DIR);

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/cosmos/auth", sdk_dir.display()),
        format!("{}/proto/cosmos/bank", sdk_dir.display()),
        format!("{}/proto/cosmos/base", sdk_dir.display()),
        format!("{}/proto/cosmos/base/tendermint", sdk_dir.display()),
        format!("{}/proto/cosmos/capability", sdk_dir.display()),
        format!("{}/proto/cosmos/crisis", sdk_dir.display()),
        format!("{}/proto/cosmos/crypto", sdk_dir.display()),
        format!("{}/proto/cosmos/distribution", sdk_dir.display()),
        format!("{}/proto/cosmos/evidence", sdk_dir.display()),
        format!("{}/proto/cosmos/genutil", sdk_dir.display()),
        format!("{}/proto/cosmos/gov", sdk_dir.display()),
        format!("{}/proto/cosmos/mint", sdk_dir.display()),
        format!("{}/proto/cosmos/params", sdk_dir.display()),
        format!("{}/proto/cosmos/slashing", sdk_dir.display()),
        format!("{}/proto/cosmos/staking", sdk_dir.display()),
        format!("{}/proto/cosmos/tx", sdk_dir.display()),
        format!("{}/proto/cosmos/upgrade", sdk_dir.display()),
        format!("{}/proto/cosmos/vesting", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}

fn compile_chain_main_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling chain_main .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");
    let sdk_dir = Path::new(CHAIN_MAIN_DIR);

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/../cosmos-sdk/proto", sdk_dir.display()),
        format!("{}/../cosmos-sdk/third_party/proto", sdk_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/chainmain", sdk_dir.display()),
        format!("{}/proto/nft", sdk_dir.display()),
        format!("{}/proto/supply", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}
/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let contents = fs::read_to_string(src)?;

    // `prost-build` output references types from `tendermint-proto` crate
    // relative paths, which we need to munge into `tendermint_proto` in
    // order to leverage types from the upstream crate.
    let contents = Regex::new(TENDERMINT_PROTO_REGEX)
        .unwrap()
        .replace_all(&contents, "tendermint_proto");

    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
