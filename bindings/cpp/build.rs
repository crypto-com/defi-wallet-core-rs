const BRIDGES: &[&str] = &["src/lib.rs", "src/nft.rs", "src/contract.rs"];

fn main() {
    cxx_build::bridges(BRIDGES)
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");

    for bridge in BRIDGES {
        println!("cargo:rerun-if-changed={}", bridge);
    }
}
