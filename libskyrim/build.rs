// libskyrim/build.rs
fn main() {
    let mut config = xmake::Config::new("cpp");
    config.build();

    let dst = config.build_info().linkdirs().first().unwrap();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=commonlib_bridge");
    println!("cargo:rustc-link-lib=static=commonlibsse-ng");
    println!("cargo:rustc-link-lib=static=spdlog");

    // === БАЗОВЫЕ БИБЛИОТЕКИ WINDOWS ===
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=advapi32");   // Для RegGetValue (Реестр)
    println!("cargo:rustc-link-lib=bcrypt");     // Для BCryptOpenAlgorithmProvider (Хэширование/Криптография)
    println!("cargo:rustc-link-lib=ole32");      // Для COM (CoTaskMemFree и т.д.)
    println!("cargo:rustc-link-lib=shell32");    // Для SHGetKnownFolderPath
    println!("cargo:rustc-link-lib=dbghelp");    // Для отладки и stack trace

    // === DIRECTX ===
    // CommonLibSSE-NG использует DirectX для рендера UI, хуков и прочего.
    println!("cargo:rustc-link-lib=d3d11");
    println!("cargo:rustc-link-lib=dxgi");
    println!("cargo:rustc-link-lib=d3dcompiler"); // Для D3DCompile

    println!("cargo:rerun-if-changed=cpp/src");
    println!("cargo:rerun-if-changed=cpp/include");
    println!("cargo:rerun-if-changed=cpp/xmake.lua");
}
