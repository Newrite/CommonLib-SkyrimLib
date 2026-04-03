#include "PCH.h"

#include "bridge_test_support.h"

#include <cstdlib>
#include <cstring>
#include <malloc.h>
#include <cwchar>
#include <string_view>

namespace
{
#ifdef ENABLE_COMMONLIBSSE_TESTING
    bool BRIDGE_RUST_TEST_HARNESS_ACTIVE = false;
#endif

    [[nodiscard]] bool bridge_is_ascii_hex(wchar_t ch) noexcept
    {
        return (ch >= L'0' && ch <= L'9') || (ch >= L'a' && ch <= L'f') ||
               (ch >= L'A' && ch <= L'F');
    }

    [[nodiscard]] bool bridge_is_rust_test_harness_name(std::wstring_view file_name) noexcept
    {
        constexpr auto exe_suffix = std::wstring_view{ L".exe", 4 };
        if (!file_name.ends_with(exe_suffix)) {
            return false;
        }

        file_name.remove_suffix(exe_suffix.size());
        const auto dash = file_name.find_last_of(L'-');
        if (dash == std::wstring_view::npos || dash == 0 || dash + 1 >= file_name.size()) {
            return false;
        }

        const auto hash = file_name.substr(dash + 1);
        if (hash.size() < 8) {
            return false;
        }

        for (const auto ch : hash) {
            if (!bridge_is_ascii_hex(ch)) {
                return false;
            }
        }

        return true;
    }

    [[nodiscard]] std::size_t bridge_normalize_alignment(std::size_t alignment) noexcept
    {
        auto normalized = sizeof(void*);
        if (alignment <= normalized) {
            return normalized;
        }

        normalized = 1;
        while (normalized < alignment) {
            normalized <<= 1;
        }
        return normalized;
    }

    [[nodiscard]] bool bridge_prepare_rust_test_harness() noexcept
    {
        if (::GetModuleHandleW(L"SkyrimSE.exe") || ::GetModuleHandleW(L"SkyrimVR.exe")) {
            return false;
        }

        wchar_t path[MAX_PATH]{};
        const auto len = ::GetModuleFileNameW(nullptr, path, static_cast<DWORD>(std::size(path)));
        if (!len || len >= std::size(path)) {
            return false;
        }

        std::wstring_view file_name(path, len);
        if (const auto separator = file_name.find_last_of(L"\\/"); separator != std::wstring_view::npos) {
            file_name.remove_prefix(separator + 1);

            const std::wstring_view full_path(path, len);
            const auto deps_component = full_path.rfind(L"\\deps\\");
            const auto is_cargo_test_exe =
                deps_component != std::wstring_view::npos &&
                bridge_is_rust_test_harness_name(file_name);
            if (file_name.starts_with(L"libskyrim-") || is_cargo_test_exe) {
                path[separator] = L'\0';
                (void)::SetCurrentDirectoryW(path);
#ifdef ENABLE_COMMONLIBSSE_TESTING
                BRIDGE_RUST_TEST_HARNESS_ACTIVE = true;
#endif
                return true;
            }
        }

        return false;
    }

#ifdef ENABLE_COMMONLIBSSE_TESTING
    struct bridge_commonlib_test_bootstrap
    {
        bridge_commonlib_test_bootstrap() noexcept
        {
            if (!bridge_prepare_rust_test_harness()) {
                return;
            }

            const auto base = reinterpret_cast<std::uintptr_t>(::GetModuleHandleW(nullptr));
            if (!base) {
                return;
            }

#if defined(ENABLE_SKYRIM_VR) && !defined(ENABLE_SKYRIM_SE) && !defined(ENABLE_SKYRIM_AE)
            const auto version = REL::Version{ 1, 4, 15, 0 };
            (void)REL::Module::mock(version, REL::Module::Runtime::VR, L"SkyrimVR.exe", base);
#elif defined(ENABLE_SKYRIM_AE)
            const auto version = REL::Version{ 1, 6, 1170, 0 };
            (void)REL::Module::mock(version, REL::Module::Runtime::AE, L"SkyrimSE.exe", base);
#else
            const auto version = REL::Version{ 1, 5, 97, 0 };
            (void)REL::Module::mock(version, REL::Module::Runtime::SE, L"SkyrimSE.exe", base);
#endif
        }
    };

    [[maybe_unused]] bridge_commonlib_test_bootstrap BRIDGE_COMMONLIB_TEST_BOOTSTRAP;
#endif
}

bool bridge_test_allocator_active() noexcept
{
#ifdef ENABLE_COMMONLIBSSE_TESTING
    return BRIDGE_RUST_TEST_HARNESS_ACTIVE;
#else
    return false;
#endif
}

void* bridge_test_malloc_bytes(std::size_t size) noexcept
{
    return std::malloc(size ? size : 1);
}

void bridge_test_free_bytes(void* ptr) noexcept
{
    std::free(ptr);
}

void* bridge_test_calloc_bytes(std::size_t count, std::size_t size) noexcept
{
    return std::calloc(count ? count : 1, size ? size : 1);
}

void* bridge_test_realloc_bytes(void* ptr, std::size_t size) noexcept
{
    return std::realloc(ptr, size ? size : 1);
}

void* bridge_test_aligned_alloc_bytes(std::size_t alignment, std::size_t size) noexcept
{
    return _aligned_malloc(size ? size : 1, bridge_normalize_alignment(alignment));
}

void bridge_test_aligned_free_bytes(void* ptr) noexcept
{
    _aligned_free(ptr);
}

namespace
{
    [[nodiscard]] char* bridge_test_duplicate_c_string(const char* string) noexcept
    {
        if (!string || !*string) {
            return nullptr;
        }

        const auto length = std::strlen(string) + 1;
        auto* duplicated = static_cast<char*>(bridge_test_malloc_bytes(length));
        if (!duplicated) {
            return nullptr;
        }

        std::memcpy(duplicated, string, length);
        return duplicated;
    }

    [[nodiscard]] const char* bridge_test_bs_fixed_string_data(const void* string) noexcept
    {
        if (!string) {
            return nullptr;
        }

        return *static_cast<const char* const*>(string);
    }

    [[nodiscard]] wchar_t* bridge_test_duplicate_w_string(const wchar_t* string) noexcept
    {
        if (!string || !*string) {
            return nullptr;
        }

        const auto length = std::wcslen(string) + 1;
        auto* duplicated =
            static_cast<wchar_t*>(bridge_test_malloc_bytes(length * sizeof(wchar_t)));
        if (!duplicated) {
            return nullptr;
        }

        std::memcpy(duplicated, string, length * sizeof(wchar_t));
        return duplicated;
    }

    [[nodiscard]] const wchar_t* bridge_test_bs_fixed_string_w_data(const void* string) noexcept
    {
        if (!string) {
            return nullptr;
        }

        return *static_cast<const wchar_t* const*>(string);
    }
}

void bridge_test_bs_fixed_string_ctor8(void* out, const char* string) noexcept
{
    if (!out) {
        return;
    }

    *static_cast<const char**>(out) = bridge_test_duplicate_c_string(string);
}

void bridge_test_bs_fixed_string_copy(void* out, const void* src) noexcept
{
    if (!out) {
        return;
    }

    *static_cast<const char**>(out) =
        bridge_test_duplicate_c_string(bridge_test_bs_fixed_string_data(src));
}

void bridge_test_bs_fixed_string_destroy(void* string) noexcept
{
    if (!string) {
        return;
    }

    auto* data = *static_cast<const char**>(string);
    bridge_test_free_bytes(const_cast<char*>(data));
    *static_cast<const char**>(string) = nullptr;
}

std::uint32_t bridge_test_bs_fixed_string_size(const void* string) noexcept
{
    const auto* data = bridge_test_bs_fixed_string_data(string);
    return data ? static_cast<std::uint32_t>(std::strlen(data)) : 0;
}

const char* bridge_test_bs_fixed_string_c_str(const void* string) noexcept
{
    const auto* data = bridge_test_bs_fixed_string_data(string);
    return data ? data : "";
}

bool bridge_test_bs_fixed_string_eq(const void* lhs, const void* rhs) noexcept
{
    const auto* lhs_data = bridge_test_bs_fixed_string_c_str(lhs);
    const auto* rhs_data = bridge_test_bs_fixed_string_c_str(rhs);
    return std::strcmp(lhs_data, rhs_data) == 0;
}

std::uint32_t bridge_test_bs_fixed_string_hash(const void* string) noexcept
{
    constexpr std::uint32_t fnv_offset = 2166136261u;
    constexpr std::uint32_t fnv_prime = 16777619u;

    auto hash = fnv_offset;
    for (const auto* current = bridge_test_bs_fixed_string_c_str(string); *current; ++current) {
        hash ^= static_cast<unsigned char>(*current);
        hash *= fnv_prime;
    }
    return hash;
}

void bridge_test_bs_fixed_string_ctor16(void* out, const wchar_t* string) noexcept
{
    if (!out) {
        return;
    }

    *static_cast<const wchar_t**>(out) = bridge_test_duplicate_w_string(string);
}

void bridge_test_bs_fixed_string_w_copy(void* out, const void* src) noexcept
{
    if (!out) {
        return;
    }

    *static_cast<const wchar_t**>(out) =
        bridge_test_duplicate_w_string(bridge_test_bs_fixed_string_w_data(src));
}

void bridge_test_bs_fixed_string_w_destroy(void* string) noexcept
{
    if (!string) {
        return;
    }

    auto* data = *static_cast<const wchar_t**>(string);
    bridge_test_free_bytes(const_cast<wchar_t*>(data));
    *static_cast<const wchar_t**>(string) = nullptr;
}

std::uint32_t bridge_test_bs_fixed_string_w_size(const void* string) noexcept
{
    const auto* data = bridge_test_bs_fixed_string_w_data(string);
    return data ? static_cast<std::uint32_t>(std::wcslen(data)) : 0;
}

const wchar_t* bridge_test_bs_fixed_string_w_c_str(const void* string) noexcept
{
    const auto* data = bridge_test_bs_fixed_string_w_data(string);
    return data ? data : L"";
}

bool bridge_test_bs_fixed_string_w_eq(const void* lhs, const void* rhs) noexcept
{
    const auto* lhs_data = bridge_test_bs_fixed_string_w_c_str(lhs);
    const auto* rhs_data = bridge_test_bs_fixed_string_w_c_str(rhs);
    return std::wcscmp(lhs_data, rhs_data) == 0;
}

std::uint32_t bridge_test_bs_fixed_string_w_hash(const void* string) noexcept
{
    constexpr std::uint32_t fnv_offset = 2166136261u;
    constexpr std::uint32_t fnv_prime = 16777619u;

    auto hash = fnv_offset;
    for (const auto* current = bridge_test_bs_fixed_string_w_c_str(string); *current; ++current) {
        hash ^= static_cast<std::uint32_t>(*current);
        hash *= fnv_prime;
    }
    return hash;
}
