fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .compile("defi_wallet_core");
}
