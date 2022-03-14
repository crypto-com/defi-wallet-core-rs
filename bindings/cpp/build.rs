fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .file("src/main.cc")
        .file("src/cronos.cc")
        .compile("defi_wallet_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/nft.rs");
    println!("cargo:rerun-if-changed=src/main.cc");
    println!("cargo:rerun-if-changed=src/cronos.cc");
}
