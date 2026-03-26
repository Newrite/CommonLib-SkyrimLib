#include "PCH.h"

#include <memory>
#include <new>

namespace
{
    using bst_event_sink_process_callback =
        std::int32_t (*)(void* ctx, const void* event, void* event_source);
    using bst_event_sink_destroy_callback = void (*)(void* ctx);

    template <class T>
    [[nodiscard]] bool bridge_emplace_vtable(T* ptr) noexcept
    {
        return RE::stl::emplace_vtable<T>(ptr);
    }

    template <class T>
    void bridge_memzero(volatile T* ptr, std::size_t size = sizeof(T)) noexcept
    {
        RE::stl::memzero(ptr, size);
    }

    template <class To, class From>
    [[nodiscard]] auto bridge_adjust_pointer(From* ptr, std::ptrdiff_t adjust) noexcept
    {
        return RE::stl::adjust_pointer<To>(ptr, adjust);
    }

    template <class To, class From>
    [[nodiscard]] To bridge_unrestricted_cast(From from) noexcept
    {
        return RE::stl::unrestricted_cast<To>(from);
    }

    template <class SmartPointer, class Factory>
    [[nodiscard]] bool construct_smart_pointer_out(SmartPointer* out, Factory&& factory) noexcept
    {
        if (!out) {
            return false;
        }

        try {
            std::construct_at(out, std::forward<Factory>(factory)());
            return true;
        } catch (...) {
            std::construct_at(out, SmartPointer{});
            return false;
        }
    }

    [[nodiscard]] RE::BSEventNotifyControl bridge_notify_control_from_i32(std::int32_t value) noexcept
    {
        return value == static_cast<std::int32_t>(RE::BSEventNotifyControl::kStop) ?
                   RE::BSEventNotifyControl::kStop :
                   RE::BSEventNotifyControl::kContinue;
    }

    class bridge_bst_event_sink final : public RE::BSTEventSink<void*>
    {
    public:
        bridge_bst_event_sink(
            void* a_ctx,
            bst_event_sink_process_callback a_process,
            bst_event_sink_destroy_callback a_destroy) noexcept :
            ctx(a_ctx),
            process(a_process),
            destroy(a_destroy)
        {}

        ~bridge_bst_event_sink() override
        {
            if (destroy && ctx) {
                destroy(ctx);
            }
        }

        RE::BSEventNotifyControl ProcessEvent(
            void* const* a_event,
            RE::BSTEventSource<void*>* a_eventSource) override
        {
            if (!process) {
                return RE::BSEventNotifyControl::kContinue;
            }

            return bridge_notify_control_from_i32(process(
                ctx,
                bridge_unrestricted_cast<const void*>(a_event),
                bridge_unrestricted_cast<void*>(a_eventSource)));
        }

        void*                           ctx;
        bst_event_sink_process_callback process;
        bst_event_sink_destroy_callback destroy;
    };
}

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

    // Выделение памяти через движок Скайрима
    void* commonlib_malloc(size_t size) {
        return RE::malloc(size);
    }

    void* commonlib_aligned_alloc(size_t alignment, size_t size) {
        return RE::aligned_alloc(alignment, size);
    }

    // Освобождение памяти
    void commonlib_free(void* ptr) {
        RE::free(ptr);
    }

    void commonlib_aligned_free(void* ptr) {
        RE::aligned_free(ptr);
    }

    void* commonlib_calloc(size_t count, size_t size) {
        return RE::calloc(count, size);
    }

    void* commonlib_realloc(void* ptr, size_t new_size) {
        return RE::realloc(ptr, new_size);
    }

    // ── MemoryManager ────────────────────────────────────────────────────────

    void* commonlib_memory_manager_get_singleton() {
        return RE::MemoryManager::GetSingleton();
    }

    void* commonlib_memory_manager_get_thread_scrap_heap(void* mgr) {
        return static_cast<RE::MemoryManager*>(mgr)->GetThreadScrapHeap();
    }

    // ── ScrapHeap ────────────────────────────────────────────────────────────

    void* commonlib_scrap_heap_allocate(void* heap, size_t size, size_t alignment) {
        return static_cast<RE::ScrapHeap*>(heap)->Allocate(size, alignment);
    }

    void commonlib_scrap_heap_deallocate(void* heap, void* mem) {
        static_cast<RE::ScrapHeap*>(heap)->Deallocate(mem);
    }

    int32_t commonlib_actor_get_gold_amount(void* actor, bool no_init) {
        return static_cast<RE::Actor*>(actor)->GetGoldAmount(no_init);
    }

    bool commonlib_make_hkref_hk_referenced_object(void* out) noexcept {
        return construct_smart_pointer_out(
            static_cast<RE::hkRefPtr<RE::hkReferencedObject>*>(out),
            []() {
                return RE::make_hkref<RE::hkReferencedObject>();
            });
    }

    bool commonlib_make_nismart_ni_ref_object(void* out) noexcept {
        return construct_smart_pointer_out(
            static_cast<RE::NiPointer<RE::NiRefObject>*>(out),
            []() {
                return RE::make_nismart<RE::NiRefObject>();
            });
    }

    void commonlib_destroy_bsi_input_device(void* device) noexcept {
        delete static_cast<RE::BSIInputDevice*>(device);
    }

    void* commonlib_button_event_create(
        std::int32_t input_device,
        const void* user_event,
        std::uint32_t id_code,
        float value,
        float held_down_secs) noexcept
    {
        const auto full_size =
            REL::Module::IsVR() ?
                sizeof(RE::VRWandEvent) + sizeof(RE::ButtonEvent::RUNTIME_DATA) :
                sizeof(RE::IDEvent) + sizeof(RE::ButtonEvent::RUNTIME_DATA);

        auto* button_event = RE::malloc<RE::ButtonEvent>(full_size);
        if (!button_event) {
            return nullptr;
        }

        bridge_memzero(button_event, full_size);
        if (!bridge_emplace_vtable(button_event)) {
            RE::free(button_event);
            return nullptr;
        }

        auto* input_event = bridge_adjust_pointer<RE::InputEvent>(button_event, 0);
        input_event->device = static_cast<RE::INPUT_DEVICE>(input_device);
        input_event->eventType = RE::INPUT_EVENT_TYPE::kButton;
        input_event->next = nullptr;

        button_event->SetUserEvent(*static_cast<const RE::BSFixedString*>(user_event));
        button_event->SetIDCode(id_code);
        button_event->GetRuntimeData().value = value;
        button_event->GetRuntimeData().heldDownSecs = held_down_secs;

        return bridge_unrestricted_cast<void*>(button_event);
    }

    void* commonlib_bst_event_sink_create(
        void* ctx,
        bst_event_sink_process_callback process,
        bst_event_sink_destroy_callback destroy) noexcept
    {
        return new (std::nothrow) bridge_bst_event_sink(ctx, process, destroy);
    }

    void commonlib_bst_event_sink_destroy(void* sink) noexcept {
        delete static_cast<bridge_bst_event_sink*>(sink);
    }

    void* commonlib_skse_get_serialization_interface() noexcept {
        return const_cast<SKSE::SerializationInterface*>(SKSE::GetSerializationInterface());
    }

    void* commonlib_skse_get_trampoline_interface() noexcept {
        return const_cast<SKSE::TrampolineInterface*>(SKSE::GetTrampolineInterface());
    }

    void* commonlib_skse_get_mod_callback_event_source() noexcept {
        return SKSE::GetModCallbackEventSource();
    }

    void* commonlib_skse_get_camera_event_source() noexcept {
        return SKSE::GetCameraEventSource();
    }

    void* commonlib_skse_get_crosshair_ref_event_source() noexcept {
        return SKSE::GetCrosshairRefEventSource();
    }

    void* commonlib_skse_get_action_event_source() noexcept {
        return SKSE::GetActionEventSource();
    }

    void* commonlib_skse_get_ni_node_update_event_source() noexcept {
        return SKSE::GetNiNodeUpdateEventSource();
    }

}
