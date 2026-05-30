use std::env;
use std::fs;
use std::path::PathBuf;

const PRIVATE: &str = "\
#[doc(hidden)]
pub mod __private$$ {
    #[doc(hidden)]
    pub use crate::private::*;
}
use serde_core::__private$$ as serde_core_private;
";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=if_docsrs_then_no_serde_core");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let patch_version = env::var("CARGO_PKG_VERSION_PATCH").unwrap();
    let module = PRIVATE.replace("$$", &patch_version);
    fs::write(out_dir.join("private.rs"), module).unwrap();

    println!("cargo:rustc-check-cfg=cfg(feature, values(\"result\"))");
    println!("cargo:rustc-check-cfg=cfg(if_docsrs_then_no_serde_core)");
    println!("cargo:rustc-check-cfg=cfg(no_core_cstr)");
    println!("cargo:rustc-check-cfg=cfg(no_core_error)");
    println!("cargo:rustc-check-cfg=cfg(no_core_net)");
    println!("cargo:rustc-check-cfg=cfg(no_core_num_saturating)");
    println!("cargo:rustc-check-cfg=cfg(no_diagnostic_namespace)");
    println!("cargo:rustc-check-cfg=cfg(no_serde_derive)");
    println!("cargo:rustc-check-cfg=cfg(no_std_atomic)");
    println!("cargo:rustc-check-cfg=cfg(no_std_atomic64)");
    println!("cargo:rustc-check-cfg=cfg(no_target_has_atomic)");
}
