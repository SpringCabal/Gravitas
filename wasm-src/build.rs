fn main() {
    println!("cargo:rustc-link-arg=--initial-memory=67108864");
    println!("cargo:rustc-link-arg=--no-growable-memory");
    println!("cargo:rerun-if-changed=build.rs");
}
