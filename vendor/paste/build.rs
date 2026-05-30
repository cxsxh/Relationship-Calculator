fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(no_literal_fromstr)");
    println!("cargo:rustc-check-cfg=cfg(feature, values(\"protocol_feature_paste\"))");
}