fn main() {
    if std::env::var("CARGO_FEATURE_TUNING").is_ok() {
        println!("cargo:rustc-cfg=feature=\"tuning\"");
    }
}
