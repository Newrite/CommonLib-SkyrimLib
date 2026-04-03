#pragma once

#include <cstddef>
#include <cstdint>

[[nodiscard]] bool bridge_test_allocator_active() noexcept;
[[nodiscard]] void* bridge_test_malloc_bytes(std::size_t size) noexcept;
void bridge_test_free_bytes(void* ptr) noexcept;
[[nodiscard]] void* bridge_test_calloc_bytes(std::size_t count, std::size_t size) noexcept;
[[nodiscard]] void* bridge_test_realloc_bytes(void* ptr, std::size_t size) noexcept;
[[nodiscard]] void* bridge_test_aligned_alloc_bytes(std::size_t alignment, std::size_t size) noexcept;
void bridge_test_aligned_free_bytes(void* ptr) noexcept;

void bridge_test_bs_fixed_string_ctor8(void* out, const char* string) noexcept;
void bridge_test_bs_fixed_string_copy(void* out, const void* src) noexcept;
void bridge_test_bs_fixed_string_destroy(void* string) noexcept;
[[nodiscard]] std::uint32_t bridge_test_bs_fixed_string_size(const void* string) noexcept;
[[nodiscard]] const char* bridge_test_bs_fixed_string_c_str(const void* string) noexcept;
[[nodiscard]] bool bridge_test_bs_fixed_string_eq(const void* lhs, const void* rhs) noexcept;
[[nodiscard]] std::uint32_t bridge_test_bs_fixed_string_hash(const void* string) noexcept;

void bridge_test_bs_fixed_string_ctor16(void* out, const wchar_t* string) noexcept;
void bridge_test_bs_fixed_string_w_copy(void* out, const void* src) noexcept;
void bridge_test_bs_fixed_string_w_destroy(void* string) noexcept;
[[nodiscard]] std::uint32_t bridge_test_bs_fixed_string_w_size(const void* string) noexcept;
[[nodiscard]] const wchar_t* bridge_test_bs_fixed_string_w_c_str(const void* string) noexcept;
[[nodiscard]] bool bridge_test_bs_fixed_string_w_eq(const void* lhs, const void* rhs) noexcept;
[[nodiscard]] std::uint32_t bridge_test_bs_fixed_string_w_hash(const void* string) noexcept;
