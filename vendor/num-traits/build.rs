fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_total_cmp)");
    println!("cargo:rustc-cfg=has_total_cmp");
    println!("cargo:rerun-if-changed=build.rs");
}