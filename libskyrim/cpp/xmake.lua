-- set minimum xmake version
set_xmakever("3.0.0")

-- Создаем локальный пакет для CommonLibSSE-NG, который собирается через сам Xmake
package("commonlibsse-ng-custom")
    set_homepage("https://github.com/alandtse/CommonLibVR")
    set_description("CommonLibVR/SSE NG branch (built via xmake)")

    -- Указываем URL репозитория
    add_urls("https://github.com/alandtse/CommonLibVR.git")

    -- Указываем ветку
    add_versions("ng", "ng")

    -- Передаем опции из оригинального xmake.lua CommonLib-а
    add_configs("skyrim_se", {description = "Enable SE", default = true, type = "boolean"})
    add_configs("skyrim_ae", {description = "Enable AE", default = true, type = "boolean"})
    add_configs("skyrim_vr", {description = "Enable VR", default = false, type = "boolean"})

    -- Если оригинальный проект требует этих пакетов для сборки (из твоего лога)
    add_deps("directxmath", "directxtk", "spdlog")

    on_install(function (package)
        local configs = {}

        -- Пробрасываем флаги в сборку CommonLib
        if package:config("skyrim_se") then table.insert(configs, "--skyrim_se=y") else table.insert(configs, "--skyrim_se=n") end
        if package:config("skyrim_ae") then table.insert(configs, "--skyrim_ae=y") else table.insert(configs, "--skyrim_ae=n") end
        if package:config("skyrim_vr") then table.insert(configs, "--skyrim_vr=y") else table.insert(configs, "--skyrim_vr=n") end

        -- ВАЖНО: Мы используем package.tools.xmake, а не cmake!
        -- Xmake скачает код, увидит внутри xmake.lua и сам соберет его!
        import("package.tools.xmake").install(package, configs)
    end)
package_end()

-- Требуем наш пакет (он соберется автоматически при первом запуске)
add_requires("commonlibsse-ng-custom")

-- Наша цель (мост)
target("commonlib_bridge")
    set_kind("static")
    set_languages("c++23")

    add_includedirs("include")
    set_pcxxheader("include/PCH.h")

    -- Подключаем собранный CommonLib
    add_packages("commonlibsse-ng-custom")

    -- Исходник моста (ffi)
    add_headerfiles("include/**.h", "include/**.hpp")
    add_files("src/**.cpp")
