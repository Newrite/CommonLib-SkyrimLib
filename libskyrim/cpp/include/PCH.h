#pragma once
#pragma execution_character_set("utf-8")


#include <RE/Skyrim.h>
#include <SKSE/SKSE.h>
#include <SKSE/API.h>


#include "spdlog/sinks/basic_file_sink.h"
#include "spdlog/pattern_formatter.h"

namespace logger = SKSE::log;
namespace stl = SKSE::stl;
using namespace std::literals;

#define DLLEXPORT __declspec(dllexport)
