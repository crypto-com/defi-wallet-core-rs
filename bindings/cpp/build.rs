use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");
}
