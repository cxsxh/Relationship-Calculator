fn main() {
    println!("cargo:rerun-if-changed=build/probe.rs");
    println!("cargo:rustc-check-cfg=cfg(error_generic_member_access)");
    println!("cargo:rustc-check-cfg=cfg(thiserror_nightly_testing)");
}