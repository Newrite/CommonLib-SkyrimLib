// libskyrim/build.rs
use std::env;

fn main() {
    let mut config = xmake::Config::new("cpp");

    // 1. Проброс режима сборки (Debug / Release)
    // Cargo передает нам текущий профиль через переменную PROFILE
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    if profile == "release" {
        config.mode("release");
    } else {
        config.mode("debug");
    }

    // 2. Проброс таргетов Скайрима через Cargo Features
    // Если фича включена, Cargo создает переменную CARGO_FEATURE_<ИМЯ>
    let is_se = env::var("CARGO_FEATURE_SE").is_ok();
    let is_ae = env::var("CARGO_FEATURE_AE").is_ok();
    let is_vr = env::var("CARGO_FEATURE_VR").is_ok();

    // Передаем опции в xmake (xmake f --skyrim_se=y ...)
    config.option("skyrim_se", if is_se { "y" } else { "n" });
    config.option("skyrim_ae", if is_ae { "y" } else { "n" });
    config.option("skyrim_vr", if is_vr { "y" } else { "n" });
    config.option("skse_xbyak", "y"); // Всегда включено для хуков

    // Запускаем сборку C++ кода
    config.build();

    let dst = config.build_info().linkdirs().first().unwrap();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=commonlib_bridge");
    println!("cargo:rustc-link-lib=static=commonlibsse-ng");
    println!("cargo:rustc-link-lib=static=spdlog");

    // === БАЗОВЫЕ БИБЛИОТЕКИ WINDOWS ===
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=bcrypt");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=dbghelp");

    // === DIRECTX ===
    println!("cargo:rustc-link-lib=d3d11");
    println!("cargo:rustc-link-lib=dxgi");
    println!("cargo:rustc-link-lib=d3dcompiler");

    // Следим за изменениями, чтобы пересобирать только при нужде
    println!("cargo:rerun-if-changed=cpp/src");
    println!("cargo:rerun-if-changed=cpp/include");
    println!("cargo:rerun-if-changed=cpp/xmake.lua");
}