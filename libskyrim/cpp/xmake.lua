-- set minimum xmake version
set_xmakever("3.0.0")

-- Глобальные правила (решает проблемы с линковкой в разных режимах)
add_rules("mode.debug", "mode.release")

-- 1. ГЛОБАЛЬНЫЕ КОНФИГУРАЦИИ ИГРЫ
set_config("skyrim_se", true)
set_config("skyrim_ae", false)
set_config("skyrim_vr", false)
set_config("skse_xbyak", true)

-- ФИКС: Объявляем дефайны глобально ДО всего остального,
-- чтобы они применились и к нашему коду, и к PCH
if get_config("skyrim_se") then add_defines("ENABLE_SKYRIM_SE=1") end
if get_config("skyrim_ae") then add_defines("ENABLE_SKYRIM_AE=1") end
if get_config("skyrim_vr") then add_defines("ENABLE_SKYRIM_VR=1") end
if get_config("skse_xbyak") then add_defines("SKSE_SUPPORT_XBYAK=1") end

-- 2. ОПРЕДЕЛЯЕМ КАСТОМНЫЙ ПАКЕТ
package("commonlibsse-ng")
    add_urls("https://github.com/alandtse/CommonLibVR.git", { branch = "ng" })

    add_deps("directxmath 2024.02", "directxtk 24.2.0")
    add_deps("spdlog v1.16.0", { configs = { header_only = false, wchar = true, std_format = true } })
    add_deps("xbyak v7.06")

    on_install(function (package)
        local configs = {}
        table.insert(configs, "--skyrim_se="  .. (get_config("skyrim_se")  and "y" or "n"))
        table.insert(configs, "--skyrim_ae="  .. (get_config("skyrim_ae")  and "y" or "n"))
        table.insert(configs, "--skyrim_vr="  .. (get_config("skyrim_vr")  and "y" or "n"))
        table.insert(configs, "--skse_xbyak=" .. (get_config("skse_xbyak") and "y" or "n"))

        import("package.tools.xmake").install(package, configs)
    end)
package_end()

-- 3. ТРЕБУЕМ НАШ ПАКЕТ
add_requires("commonlibsse-ng")

-- 4. НАША ЦЕЛЬ (C++ Мост для Rust)
target("commonlib_bridge")
    set_kind("static")
    set_languages("c++23")

    -- ФИКС MSVC: Включаем новый препроцессор и строгое соответствие C++
    if is_plat("windows") then
        add_cxxflags("cl::/Zc:preprocessor", "cl::/permissive-", "cl::/EHsc")
    end

    -- Отключаем предупреждения
    add_cxxflags("/wd4005", "/wd4061", "/wd4068", "/wd4200", "/wd4201")

    add_includedirs("include")
    set_pcxxheader("include/PCH.h")

    -- Подключаем пакет
    add_packages("commonlibsse-ng")

    -- Исходники моста
    add_headerfiles("include/**.h", "include/**.hpp")
    add_files("src/**.cpp")
