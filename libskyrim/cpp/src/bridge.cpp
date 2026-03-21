#include "PCH.h"

extern "C" {
    // 1. Инициализация CommonLib
    void init_commonlib(const void* skse_interface) {
        SKSE::Init((const SKSE::LoadInterface*)skse_interface);
    }

    // 2. Получение адреса
    uintptr_t commonlib_id_to_address(size_t id) {
        return REL::ID(id).address();
    }

    // 3. Прямая запись байт
    void commonlib_safe_write(uintptr_t addr, const uint8_t* data, size_t len) {
        REL::safe_write(addr, data, len);
    }

    // 4. Трамплины
    uintptr_t commonlib_write_branch5(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_branch<5>(src, dst);
    }

    uintptr_t commonlib_write_call5(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_call<5>(src, dst);
    }

    // Выделение памяти под трамплин
    void commonlib_alloc_trampoline(size_t size) {
        SKSE::AllocTrampoline(size);
    }
}
