fn main() {
    #[cfg(feature = "uniffi-binding")]
    uniffi_build::generate_scaffolding("./src/common.udl").unwrap();
}
