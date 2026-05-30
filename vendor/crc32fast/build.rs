fn main() {
    println!("cargo:rustc-check-cfg=cfg(stable_arm_crc32_intrinsics)");
    println!("cargo:rustc-cfg=stable_arm_crc32_intrinsics");
}