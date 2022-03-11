fn main() {
    cxx_build::bridges(&["src/lib.rs", "src/nft.rs"])
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");
}
