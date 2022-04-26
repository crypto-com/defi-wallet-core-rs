const BRIDGES: &[&str] = &["src/lib.rs", "src/nft.rs", "src/contract.rs", "src/uint.rs"];

fn main() {
    #[cfg(not(feature = "doxygen"))]
    cxx_build::bridges(BRIDGES)
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");

    #[cfg(feature = "doxygen")]
    cxx_build_with_doxygen::bridges(BRIDGES)
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");

    for bridge in BRIDGES {
        println!("cargo:rerun-if-changed={}", bridge);
    }
}
