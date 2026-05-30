fn main() {
    println!("cargo:rustc-check-cfg=cfg(tuple_ty)");
    println!("cargo:rustc-check-cfg=cfg(allow_clippy)");
    println!("cargo:rustc-check-cfg=cfg(maybe_uninit)");
    println!("cargo:rustc-check-cfg=cfg(doctests)");
    println!("cargo:rustc-check-cfg=cfg(raw_ref_macros)");
    println!("cargo:rustc-check-cfg=cfg(stable_const)");
    println!("cargo:rustc-check-cfg=cfg(stable_offset_of)");
    println!("cargo:rustc-cfg=tuple_ty");
    println!("cargo:rustc-cfg=allow_clippy");
    println!("cargo:rustc-cfg=maybe_uninit");
    println!("cargo:rustc-cfg=doctests");
    println!("cargo:rustc-cfg=raw_ref_macros");
    println!("cargo:rustc-cfg=stable_const");
    println!("cargo:rustc-cfg=stable_offset_of");
}
