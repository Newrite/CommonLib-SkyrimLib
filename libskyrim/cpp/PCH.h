#pragma once

// Обязательно для правильной работы строк в CommonLib
#pragma execution_character_set("utf-8")

// Подключаем базовые вещи SKSE и Skyrim
#include <RE/Skyrim.h>
#include <SKSE/SKSE.h>
#include <SKSE/API.h>

// Создаем алиасы, которые ожидает код CommonLib
namespace logger = SKSE::log;
namespace stl = SKSE::stl;

using namespace std::literals;

#define DLLEXPORT __declspec(dllexport)
