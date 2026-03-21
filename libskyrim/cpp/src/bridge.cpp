#include "PCH.h"

extern "C" {
    // 1. Инициализация CommonLib
    void init_commonlib(const void* skse_interface) {
        SKSE::Init((const SKSE::LoadInterface*)skse_interface);
    }

    // 2. Получение адресов
    uintptr_t commonlib_id_to_address(size_t id) {
        return REL::ID(id).address();
    }

    uintptr_t commonlib_offset_to_address(size_t offset) {
        return REL::Offset(offset).address();
    }

    // 3. Прямая запись в память
    void commonlib_safe_write(uintptr_t addr, const void* data, size_t len) {
        REL::safe_write(addr, data, len);
    }

    void commonlib_safe_fill(uintptr_t addr, uint8_t value, size_t len) {
        REL::safe_fill(addr, value, len);
    }

    // 4. Трамплины (Хуки)
    uintptr_t commonlib_write_branch5(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_branch<5>(src, dst);
    }

    uintptr_t commonlib_write_branch6(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_branch<6>(src, dst);
    }

    uintptr_t commonlib_write_call5(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_call<5>(src, dst);
    }

    uintptr_t commonlib_write_call6(uintptr_t src, uintptr_t dst) {
        return SKSE::GetTrampoline().write_call<6>(src, dst);
    }

    // Создает общий пул памяти (вызывается 1 раз при старте)
    void commonlib_alloc_trampoline(size_t size) {
        SKSE::AllocTrampoline(size);
    }

    // Выделяет кусок памяти ИЗ пула для записи кастомного ассемблера
    void* commonlib_trampoline_allocate(size_t size) {
        return SKSE::GetTrampoline().allocate(size);
    }

    // 5. Замена виртуальных функций (VTable)
    uintptr_t commonlib_write_vfunc(uintptr_t vtable_addr, size_t idx, uintptr_t new_func) {
        // Оборачиваем адрес в REL::Relocation для доступа к write_vfunc
        REL::Relocation<uintptr_t> vtable(vtable_addr);
        return vtable.write_vfunc(idx, new_func);
    }

    // 6. Задачи (Task Interface)
    void commonlib_add_task(void (*cb)(void*), void* data) {
        // Захватываем указатели по значению [=] и вызываем внутри задачи
        SKSE::GetTaskInterface()->AddTask([=]() {
            cb(data);
        });
    }

    void commonlib_add_ui_task(void (*cb)(void*), void* data) {
        SKSE::GetTaskInterface()->AddUITask([=]() {
            cb(data);
        });
    }

}
