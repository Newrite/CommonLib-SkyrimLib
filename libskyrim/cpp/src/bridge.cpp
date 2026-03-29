#include "PCH.h"

#include <MinHook.h>
#include <memory>
#include <new>

namespace
{
    using bst_event_sink_process_callback =
        std::int32_t (*)(void* ctx, const void* event, void* event_source);
    using function_arguments_collect_callback = bool (*)(void* ctx, void* dst);
    using bst_event_sink_destroy_callback = void (*)(void* ctx);
    using native_function_marshall_callback =
        bool (*)(void* ctx, void* base_value, void* vm, std::uint32_t stack_id, void* result_value, const void* frame);

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

    template <class T, class... Args>
    [[nodiscard]] T* construct_game_object(Args&&... args) noexcept
    {
        auto* storage = RE::malloc<T>();
        if (!storage) {
            return nullptr;
        }

        try {
            return std::construct_at(storage, std::forward<Args>(args)...);
        } catch (...) {
            RE::free(storage);
            return nullptr;
        }
    }

    template <class T>
    void bridge_gptr_add_ref(void* ptr) noexcept
    {
        if (ptr) {
            static_cast<T*>(ptr)->AddRef();
        }
    }

    template <class T>
    void bridge_gptr_release(void* ptr) noexcept
    {
        if (ptr) {
            static_cast<T*>(ptr)->Release();
        }
    }

    [[nodiscard]] RE::BSEventNotifyControl bridge_notify_control_from_i32(std::int32_t value) noexcept
    {
        return value == static_cast<std::int32_t>(RE::BSEventNotifyControl::kStop) ?
                   RE::BSEventNotifyControl::kStop :
                   RE::BSEventNotifyControl::kContinue;
    }

    [[nodiscard]] bool ensure_minhook_initialized() noexcept
    {
        const auto status = MH_Initialize();
        if (status == MH_OK || status == MH_ERROR_ALREADY_INITIALIZED) {
            return true;
        }

        logger::error("MinHook initialization failed: {}", MH_StatusToString(status));
        return false;
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

    class bridge_function_arguments final : public RE::BSScript::IFunctionArguments
    {
    public:
        bridge_function_arguments(
            void* a_ctx,
            function_arguments_collect_callback a_collect,
            bst_event_sink_destroy_callback a_destroy) noexcept :
            ctx(a_ctx),
            collect(a_collect),
            destroy(a_destroy)
        {}

        ~bridge_function_arguments() override
        {
            if (destroy && ctx) {
                destroy(ctx);
            }
        }

        bool operator()(RE::BSScrapArray<RE::BSScript::Variable>& a_dst) const override
        {
            if (!collect) {
                return false;
            }

            return collect(ctx, std::addressof(a_dst));
        }

        void*                             ctx;
        function_arguments_collect_callback collect;
        bst_event_sink_destroy_callback   destroy;
    };

    class bridge_native_function final : public RE::BSScript::NF_util::NativeFunctionBase
    {
    public:
        bridge_native_function(
            void* a_ctx,
            native_function_marshall_callback a_marshall,
            bst_event_sink_destroy_callback a_destroy,
            std::string_view a_fnName,
            std::string_view a_className,
            bool a_isStatic,
            const RE::BSScript::TypeInfo& a_returnType,
            const RE::BSScript::TypeInfo* a_paramTypes,
            std::uint16_t a_paramCount,
            bool a_isLatent) :
            RE::BSScript::NF_util::NativeFunctionBase(a_fnName, a_className, a_isStatic, a_paramCount),
            ctx(a_ctx),
            marshall(a_marshall),
            destroy(a_destroy)
        {
            for (std::uint16_t i = 0; i < a_paramCount; ++i) {
                _descTable.entries[i].second = a_paramTypes[i];
            }
            _retType = a_returnType;
            _isLatent = a_isLatent;
        }

        ~bridge_native_function() override
        {
            if (destroy && ctx) {
                destroy(ctx);
            }
        }

        bool HasStub() const override
        {
            return static_cast<bool>(marshall);
        }

        bool MarshallAndDispatch(
            RE::BSScript::Variable& a_baseValue,
            RE::BSScript::Internal::VirtualMachine& a_vm,
            RE::VMStackID a_stackID,
            RE::BSScript::Variable& a_resultValue,
            const RE::BSScript::StackFrame& a_frame) const override
        {
            if (!marshall) {
                return false;
            }

            return marshall(
                ctx,
                std::addressof(a_baseValue),
                std::addressof(a_vm),
                a_stackID,
                std::addressof(a_resultValue),
                std::addressof(a_frame));
        }

        void*                            ctx;
        native_function_marshall_callback marshall;
        bst_event_sink_destroy_callback  destroy;
    };
}

extern "C" {
    // 1. Инициализация CommonLib
    void init_commonlib(const void* skse_interface) {
        SKSE::Init((const SKSE::LoadInterface*)skse_interface);
    }

    void init_commonlib_with_log(const void* skse_interface, bool log) {
        SKSE::Init((const SKSE::LoadInterface*)skse_interface, log);
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

    uintptr_t commonlib_write_function_hook_universal(uintptr_t target, uintptr_t dst) {
        if (!ensure_minhook_initialized()) {
            return 0;
        }

        void* original = nullptr;
        const auto create_status =
            MH_CreateHook(reinterpret_cast<void*>(target), reinterpret_cast<void*>(dst), std::addressof(original));
        if (create_status != MH_OK) {
            logger::error(
                "MinHook create failed for target {:#016X}: {}",
                target,
                MH_StatusToString(create_status));
            return 0;
        }

        const auto enable_status = MH_EnableHook(reinterpret_cast<void*>(target));
        if (enable_status != MH_OK && enable_status != MH_ERROR_ENABLED) {
            logger::error(
                "MinHook enable failed for target {:#016X}: {}",
                target,
                MH_StatusToString(enable_status));
            MH_RemoveHook(reinterpret_cast<void*>(target));
            return 0;
        }

        return reinterpret_cast<uintptr_t>(original);
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

    void commonlib_raise_seh_exception(std::uint32_t code) noexcept {
        ::RaiseException(code, EXCEPTION_NONCONTINUABLE, 0, nullptr);
        ::TerminateProcess(::GetCurrentProcess(), code);
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

    void commonlib_gfx_movie_view_add_ref(void* movie_view) noexcept {
        bridge_gptr_add_ref<RE::GFxMovieView>(movie_view);
    }

    void commonlib_gfx_movie_view_release(void* movie_view) noexcept {
        bridge_gptr_release<RE::GFxMovieView>(movie_view);
    }

    void commonlib_gfx_resource_delete(void* resource) noexcept {
        delete static_cast<RE::GFxResource*>(resource);
    }

    void commonlib_fx_delegate_add_ref(void* delegate) noexcept {
        bridge_gptr_add_ref<RE::FxDelegate>(delegate);
    }

    void commonlib_fx_delegate_release(void* delegate) noexcept {
        bridge_gptr_release<RE::FxDelegate>(delegate);
    }

    void commonlib_fx_delegate_handler_add_ref(void* handler) noexcept {
        bridge_gptr_add_ref<RE::FxDelegateHandler>(handler);
    }

    void commonlib_fx_delegate_handler_release(void* handler) noexcept {
        bridge_gptr_release<RE::FxDelegateHandler>(handler);
    }

    void commonlib_imenu_add_ref(void* menu) noexcept {
        bridge_gptr_add_ref<RE::IMenu>(menu);
    }

    void commonlib_imenu_release(void* menu) noexcept {
        bridge_gptr_release<RE::IMenu>(menu);
    }

    void* commonlib_bgs_attack_data_create() noexcept {
        return RE::BGSAttackData::Create();
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

    void* commonlib_function_arguments_create(
        void* ctx,
        function_arguments_collect_callback collect,
        bst_event_sink_destroy_callback destroy) noexcept
    {
        return construct_game_object<bridge_function_arguments>(ctx, collect, destroy);
    }

    void* commonlib_function_arguments_create_zero() noexcept {
        return construct_game_object<RE::BSScript::ZeroFunctionArguments>();
    }

    void commonlib_function_arguments_destroy(void* args) noexcept {
        delete static_cast<RE::BSScript::IFunctionArguments*>(args);
    }

    void* commonlib_native_function_create(
        void* ctx,
        native_function_marshall_callback marshall,
        bst_event_sink_destroy_callback destroy,
        const char* fn_name,
        const char* class_name,
        bool is_static,
        const void* return_type,
        const void* param_types,
        std::size_t param_count,
        bool is_latent) noexcept
    {
        if (!fn_name || !class_name || !return_type || param_count > 0xFFFF ||
            (param_count > 0 && !param_types)) {
            return nullptr;
        }

        auto* param_type_info =
            static_cast<const RE::BSScript::TypeInfo*>(param_types);
        return construct_game_object<bridge_native_function>(
            ctx,
            marshall,
            destroy,
            fn_name,
            class_name,
            is_static,
            *static_cast<const RE::BSScript::TypeInfo*>(return_type),
            param_type_info,
            static_cast<std::uint16_t>(param_count),
            is_latent);
    }

    void commonlib_native_function_destroy(void* function) noexcept {
        delete static_cast<bridge_native_function*>(function);
    }

    void* commonlib_skse_get_scaleform_interface() noexcept {
        return const_cast<SKSE::ScaleformInterface*>(SKSE::GetScaleformInterface());
    }

    void* commonlib_skse_get_serialization_interface() noexcept {
        return const_cast<SKSE::SerializationInterface*>(SKSE::GetSerializationInterface());
    }

    void* commonlib_skse_get_papyrus_interface() noexcept {
        return const_cast<SKSE::PapyrusInterface*>(SKSE::GetPapyrusInterface());
    }

    void* commonlib_skse_get_task_interface() noexcept {
        return const_cast<SKSE::TaskInterface*>(SKSE::GetTaskInterface());
    }

    void* commonlib_skse_get_messaging_interface() noexcept {
        return const_cast<SKSE::MessagingInterface*>(SKSE::GetMessagingInterface());
    }

    void* commonlib_skse_get_object_interface() noexcept {
        return const_cast<SKSE::ObjectInterface*>(SKSE::GetObjectInterface());
    }

    void* commonlib_skse_get_trampoline_interface() noexcept {
        return const_cast<SKSE::TrampolineInterface*>(SKSE::GetTrampolineInterface());
    }

    void* commonlib_skse_get_delay_functor_manager() noexcept {
        return const_cast<SKSEDelayFunctorManager*>(SKSE::GetDelayFunctorManager());
    }

    void* commonlib_skse_get_object_registry() noexcept {
        return const_cast<SKSEObjectRegistry*>(SKSE::GetObjectRegistry());
    }

    void* commonlib_skse_get_persistent_object_storage() noexcept {
        return const_cast<SKSEPersistentObjectStorage*>(SKSE::GetPersistentObjectStorage());
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

    void commonlib_skse_translation_parse_translation(const char* name) {
        if (!name) {
            return;
        }

        SKSE::Translation::ParseTranslation(name);
    }

    std::size_t commonlib_skse_translation_translate(
        const char* key,
        char* out_buf,
        std::size_t out_buf_len) noexcept
    {
        if (!key) {
            return 0;
        }

        std::string result;
        if (!SKSE::Translation::Translate(key, result)) {
            return 0;
        }

        const auto required = result.size() + 1;
        if (!out_buf || out_buf_len < required) {
            return required;
        }

        std::memcpy(out_buf, result.c_str(), required);
        return required;
    }

    uintptr_t commonlib_skse_iat_get_addr(const char* dll, const char* function) {
        if (!dll || !function) {
            return 0;
        }

        return SKSE::GetIATAddr(dll, function);
    }

    uintptr_t commonlib_skse_iat_get_addr_for_module(
        void* module,
        const char* dll,
        const char* function)
    {
        if (!module || !dll || !function) {
            return 0;
        }

        return SKSE::GetIATAddr(
            static_cast<REX::W32::HMODULE>(module),
            dll,
            function);
    }

    uintptr_t commonlib_skse_iat_patch(
        uintptr_t new_func,
        const char* dll,
        const char* function)
    {
        if (!dll || !function) {
            return 0;
        }

        return SKSE::PatchIAT(new_func, dll, function);
    }

}
