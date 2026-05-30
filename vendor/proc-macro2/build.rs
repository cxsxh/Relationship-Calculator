#![allow(unknown_lints)]
#![allow(unexpected_cfgs)]
#![allow(clippy::uninlined_format_args)]

fn main() {
    println!("cargo:rustc-check-cfg=cfg(fuzzing)");
    println!("cargo:rustc-check-cfg=cfg(no_is_available)");
    println!("cargo:rustc-check-cfg=cfg(no_literal_byte_character)");
    println!("cargo:rustc-check-cfg=cfg(no_literal_c_string)");
    println!("cargo:rustc-check-cfg=cfg(no_source_text)");
    println!("cargo:rustc-check-cfg=cfg(proc_macro_span)");
    println!("cargo:rustc-check-cfg=cfg(proc_macro_span_file)");
    println!("cargo:rustc-check-cfg=cfg(proc_macro_span_location)");
    println!("cargo:rustc-check-cfg=cfg(procmacro2_backtrace)");
    println!("cargo:rustc-check-cfg=cfg(procmacro2_build_probe)");
    println!("cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)");
    println!("cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)");
    println!("cargo:rustc-check-cfg=cfg(randomize_layout)");
    println!("cargo:rustc-check-cfg=cfg(span_locations)");
    println!("cargo:rustc-check-cfg=cfg(super_unstable)");
    println!("cargo:rustc-check-cfg=cfg(wrap_proc_macro)");

    let semver_exempt = cfg!(procmacro2_semver_exempt);
    if semver_exempt {
        println!("cargo:rustc-cfg=procmacro2_semver_exempt");
    }
    if semver_exempt || cfg!(feature = "span-locations") {
        println!("cargo:rustc-cfg=span_locations");
    }
    if !cfg!(feature = "proc-macro") {
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }
    println!("cargo:rustc-cfg=wrap_proc_macro");
    println!("cargo:rustc-cfg=proc_macro_span_location");
    println!("cargo:rustc-cfg=proc_macro_span_file");
    println!("cargo:rerun-if-changed=build.rs");
}