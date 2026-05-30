use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let sanitizers = env::var("CARGO_CFG_SANITIZE").unwrap_or_default();
    if sanitizers.contains("memory") {
        println!("cargo:rustc-cfg=getrandom_msan");
    }
}