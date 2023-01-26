const BRIDGES: &[&str] = &[
    "src/lib.rs",
    "src/nft.rs",
    "src/contract.rs",
    "src/ethereum.rs",
    "src/uint.rs",
];

// condition compilation for android not working
// #[cfg(target_os = "android")]
// #[cfg(not(target_os = "android"))]
// used env var TARGET instead
fn main() {
    cxx_build::CFG.doxygen = true;
    let mut command = cxx_build::bridges(BRIDGES);

    command.file("src/nft.cc");

    let target = std::env::var("TARGET").unwrap();
    let is_android = target.contains("android");
    if is_android {
        command.file("src/android.cc");
    }
    command.flag_if_supported("-std=c++11");
    command.compile("defi_wallet_core");

    for bridge in BRIDGES {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    println!("cargo:rerun-if-changed=src/nft.cc");
    if is_android {
        println!("cargo:rerun-if-changed=src/android.cc");
    }
    println!("cargo:rerun-if-changed=include/nft.h");
}
