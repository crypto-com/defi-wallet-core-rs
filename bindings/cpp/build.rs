fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .file("../../example/cpp-example/main.cc")
        .file("../../example/cpp-example/cronos.cc")
        .compile("defi_wallet_core");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/nft.rs");
    println!("cargo:rerun-if-changed=../../example/cpp-example/main.cc");
    println!("cargo:rerun-if-changed=../../example/cpp-example/cronos.cc");
}
