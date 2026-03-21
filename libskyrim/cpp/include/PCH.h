#pragma once

// Обязательно для правильной работы строк в CommonLib
#pragma execution_character_set("utf-8")

#include <cstdint>
#include <cstddef>
#include <array>
#include <string>
#include <string_view>
#include <stdexcept>
#include <compare>
#include <optional>
#include <format>
#include <filesystem>
#include <atomic>
#include <mutex>
#include <memory>
#include <vector>
#include <utility>
#include <type_traits>
#include <variant>
#include <source_location>

// Подключаем базовые вещи SKSE и Skyrim
#include <RE/Skyrim.h>
#include <SKSE/SKSE.h>
#include <SKSE/API.h>

// Создаем алиасы, которые ожидает код CommonLib
namespace logger = SKSE::log;
namespace stl = SKSE::stl;

using namespace std::literals;

#define DLLEXPORT __declspec(dllexport)
