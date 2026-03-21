// libskyrim/build.rs
fn main() {
    let mut config = xmake::Config::new("cpp");
    config.build();

    let dst = config.build_info().linkdirs().first().unwrap();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=commonlib_bridge");

    // Xmake-пакеты тоже линкуются статически. Укажем Rust найти их
    println!("cargo:rustc-link-lib=static=commonlibsse-ng");

    // fmt обычно уже включен в spdlog или линкуется автоматически,
    // но если возникнут undefined references, оставьте
    println!("cargo:rustc-link-lib=static=fmt");
    println!("cargo:rustc-link-lib=static=spdlog");

    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=user32");

    println!("cargo:rerun-if-changed=cpp/src/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/xmake.lua");
}
