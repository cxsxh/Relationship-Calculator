use std::env::VarError;
use std::env;

const ALLOWED_CFGS: &[&str] = &[
    "emscripten_old_stat_abi",
    "espidf_picolibc",
    "espidf_time32",
    "freebsd10",
    "freebsd11",
    "freebsd12",
    "freebsd13",
    "freebsd14",
    "freebsd15",
    "gnu_file_offset_bits64",
    "gnu_time_bits64",
    "libc_deny_warnings",
    "linux_time_bits64",
    "musl_v1_2_3",
    "musl32_time64",
    "musl_redir_time64",
    "vxworks_lt_25_09",
];

const CHECK_CFG_EXTRA: &[(&str, &[&str])] = &[
    (
        "target_os",
        &["switch", "aix", "ohos", "hurd", "rtems", "visionos", "nuttx", "cygwin", "qurt"],
    ),
    (
        "target_env",
        &["illumos", "wasi", "aix", "ohos", "nto71_iosock", "nto80"],
    ),
    (
        "target_arch",
        &["loongarch64", "mips32r6", "mips64r6", "csky"],
    ),
];

const MUSL_REDIR_TIME64_ARCHES: &[&str] = &["arm", "mips", "powerpc", "x86"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let libc_ci = env_flag("LIBC_CI");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_ptr_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let mut musl_v1_2_3 = env_flag("RUST_LIBC_UNSTABLE_MUSL_V1_2_3");
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_MUSL_V1_2_3");

    let musl = target_env == "musl" || target_env == "ohos";
    if target_arch == "loongarch64" || target_arch == "hexagon" || target_env == "ohos" {
        musl_v1_2_3 = true;
    }

    if musl && musl_v1_2_3 {
        set_cfg("musl_v1_2_3");
        if target_ptr_width == "32" {
            set_cfg("musl32_time64");
            set_cfg("linux_time_bits64");
        }
        if MUSL_REDIR_TIME64_ARCHES.contains(&target_arch.as_str()) {
            set_cfg("musl_redir_time64");
        }
    }

    let linux_time_bits64 = env::var("RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64").is_ok();
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64");
    if linux_time_bits64 {
        set_cfg("linux_time_bits64");
    }

    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS");
    println!("cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_GNU_TIME_BITS");
    if target_env == "gnu"
        && target_os == "linux"
        && target_ptr_width == "32"
        && target_arch != "riscv32"
        && target_arch != "x86_64"
    {
        let defaultbits = "32".to_string();
        let (timebits, filebits) = match (
            env::var("RUST_LIBC_UNSTABLE_GNU_TIME_BITS"),
            env::var("RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS"),
        ) {
            (Ok(_), Ok(_)) => panic!("Do not set both RUST_LIBC_UNSTABLE_GNU_TIME_BITS and RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS"),
            (Err(_), Err(_)) => (defaultbits.clone(), defaultbits.clone()),
            (Ok(tb), Err(_)) if tb == "64" => (tb.clone(), tb.clone()),
            (Ok(tb), Err(_)) if tb == "32" => (tb, defaultbits.clone()),
            (Ok(_), Err(_)) => panic!("Invalid value for RUST_LIBC_UNSTABLE_GNU_TIME_BITS, must be 32 or 64"),
            (Err(_), Ok(fb)) if fb == "32" || fb == "64" => (defaultbits.clone(), fb),
            (Err(_), Ok(_)) => panic!("Invalid value for RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS, must be 32 or 64"),
        };
        if timebits == "64" {
            set_cfg("linux_time_bits64");
            set_cfg("gnu_time_bits64");
        }
        if filebits == "64" {
            set_cfg("gnu_file_offset_bits64");
        }
    }

    if libc_ci {
        set_cfg("libc_deny_warnings");
    }

    for cfg in ALLOWED_CFGS {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }
    for &(name, values) in CHECK_CFG_EXTRA {
        let values = values.join("\",\"");
        println!("cargo:rustc-check-cfg=cfg({name},values(\"{values}\"))");
    }
}

fn set_cfg(cfg: &str) {
    assert!(ALLOWED_CFGS.contains(&cfg));
    println!("cargo:rustc-cfg={cfg}");
}

fn env_flag(key: &str) -> bool {
    match env::var(key) {
        Ok(x) if x == "0" => false,
        Err(VarError::NotPresent) => false,
        Err(VarError::NotUnicode(_)) => panic!("non-unicode var for `{key}`"),
        Ok(_) => true,
    }
}