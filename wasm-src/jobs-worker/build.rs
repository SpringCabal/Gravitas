fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo:rustc-link-arg=--initial-memory=8388608");
        println!("cargo:rustc-link-arg=--no-growable-memory");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
