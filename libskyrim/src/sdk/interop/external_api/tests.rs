use super::{
    ExportedCallbackRegistrar, ExportedService, InterfaceLoaderCommand, InterfaceLoaderRequest,
    InterfaceLoaderResponse, InterfaceLoaderResponseKind, InterfaceLoaderResponsePayload,
    InterfaceProvider, VersionedServiceRegistry, export_api, export_api_option,
    interface_loader_request_from_message_mut, interface_loader_response_from_message,
    plugin_dll_name, reply_interface_loader_error, select_api_for_version, select_api_for_versions,
};
use crate::sdk::events::skse::messages::MessageRef;
use crate::skse::Message;
use alloc::vec::Vec;
use spin::Mutex;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
struct TestApi {
    version: u32,
}

static API_V1: TestApi = TestApi { version: 1 };
static API_V2: TestApi = TestApi { version: 2 };
static VERSIONED_SERVICES: VersionedServiceRegistry<u32, TestApi, 2> =
    VersionedServiceRegistry::new([(1u32, &API_V1), (2u32, &API_V2)]);
static CALLBACK_TEST_GUARD: Mutex<()> = Mutex::new(());
static CALLBACK_UNREGISTRATIONS: Mutex<Vec<u64>> = Mutex::new(Vec::new());

type TestCallback = unsafe extern "system" fn(u32) -> u32;

unsafe extern "system" fn sdk_external_api_test_callback(arg: u32) -> u32 {
    arg + 1
}

unsafe extern "system" fn sdk_external_api_test_register_callback(callback: TestCallback) -> u64 {
    unsafe { callback(41) as u64 }
}

unsafe extern "system" fn sdk_external_api_test_unregister_callback(id: u64) {
    CALLBACK_UNREGISTRATIONS.lock().push(id);
}

unsafe extern "system" fn sdk_external_api_test_subscriber(id: u64) {
    CALLBACK_UNREGISTRATIONS.lock().push(id);
}

crate::export_plugin_api! {
    fn sdk_external_api_test_request(version: u32) -> TestApi {
        select_api_for_versions(version, &[(1u32, &API_V1), (2u32, &API_V2)])
    }
}

crate::export_plugin_api! {
    as "SdkExternalApiTestCustomRequest"
    fn sdk_external_api_test_custom_request(version: u32) -> TestApi {
        select_api_for_version(version, 7u32, &API_V2)
    }
}

crate::export_plugin_symbol! {
    fn sdk_external_api_test_symbol() -> TestApi {
        Some(&API_V1)
    }
}

crate::export_plugin_symbol! {
    as "SdkExternalApiTestCustomSymbol"
    fn sdk_external_api_test_custom_symbol() -> TestApi {
        Some(&API_V2)
    }
}

crate::export_plugin_symbol! {
    fn sdk_external_api_test_missing_symbol() -> TestApi {
        None
    }
}

crate::define_plugin_symbols! {
    struct Kernel32ExternalApiFlatSymbols {
        current_process_id: unsafe extern "system" fn() -> u32 = "GetCurrentProcessId";
        current_thread_id: unsafe extern "system" fn() -> u32 = "GetCurrentThreadId";
        optional maybe_missing_export: unsafe extern "system" fn() -> u32 = "SdkExternalApiDefinitelyMissing";
    }
}

#[test]
fn plugin_dll_name_adds_extension_once() {
    assert_eq!(plugin_dll_name("ExamplePlugin"), "ExamplePlugin.dll");
    assert_eq!(plugin_dll_name("ExamplePlugin.dll"), "ExamplePlugin.dll");
}

#[test]
fn export_api_option_maps_none_to_null() {
    assert!(export_api_option::<TestApi>(None).is_null());
    assert_eq!(
        export_api(&API_V1).cast::<TestApi>(),
        &API_V1 as *const _ as *mut _
    );
}

#[test]
fn exported_service_wraps_optional_single_service() {
    let available = ExportedService::new(&API_V1);
    assert!(available.is_available());
    assert_eq!(available.get().map(|api| api.version), Some(1));
    assert_eq!(
        available.export().cast::<TestApi>(),
        &API_V1 as *const _ as *mut _
    );

    let unavailable = ExportedService::<TestApi>::unavailable();
    assert!(!unavailable.is_available());
    assert!(unavailable.get().is_none());
    assert!(unavailable.export().is_null());
}

#[test]
fn select_api_helpers_match_requested_versions() {
    assert_eq!(
        select_api_for_version(1u32, 1u32, &API_V1).map(|api| api.version),
        Some(1)
    );
    assert!(select_api_for_version(1u32, 2u32, &API_V1).is_none());

    assert_eq!(
        select_api_for_versions(2u32, &[(1u32, &API_V1), (2u32, &API_V2)]).map(|api| api.version),
        Some(2)
    );
    assert!(select_api_for_versions(3u32, &[(1u32, &API_V1), (2u32, &API_V2)]).is_none());
}

#[test]
fn versioned_service_registry_selects_and_exports_versions() {
    assert_eq!(VERSIONED_SERVICES.len(), 2);
    assert!(!VERSIONED_SERVICES.is_empty());
    assert_eq!(VERSIONED_SERVICES.select(1).map(|api| api.version), Some(1));
    assert_eq!(VERSIONED_SERVICES.select(2).map(|api| api.version), Some(2));
    assert!(VERSIONED_SERVICES.select(3).is_none());
    assert!(VERSIONED_SERVICES.supports(2));
    assert!(!VERSIONED_SERVICES.supports(9));
    assert_eq!(
        VERSIONED_SERVICES.export(2).cast::<TestApi>(),
        &API_V2 as *const _ as *mut _
    );
    assert!(VERSIONED_SERVICES.export(9).is_null());
}

#[test]
fn exported_callback_registrar_unregisters_on_drop() {
    let _guard = CALLBACK_TEST_GUARD.lock();
    CALLBACK_UNREGISTRATIONS.lock().clear();

    let registrar = ExportedCallbackRegistrar::new(
        sdk_external_api_test_register_callback,
        sdk_external_api_test_unregister_callback,
    );
    {
        let registration = unsafe { registrar.register(sdk_external_api_test_callback) };
        assert_eq!(registration.id(), Some(&42));
    }

    assert_eq!(CALLBACK_UNREGISTRATIONS.lock().as_slice(), &[42]);
}

#[test]
fn exported_callback_registration_forget_skips_auto_unregistration() {
    let _guard = CALLBACK_TEST_GUARD.lock();
    CALLBACK_UNREGISTRATIONS.lock().clear();

    let registrar = ExportedCallbackRegistrar::new(
        sdk_external_api_test_register_callback,
        sdk_external_api_test_unregister_callback,
    );
    let registration = unsafe { registrar.register(sdk_external_api_test_callback) };
    assert_eq!(registration.forget(), Some(42));

    assert!(CALLBACK_UNREGISTRATIONS.lock().is_empty());
}

#[test]
fn exported_subscriber_invokes_flat_symbol() {
    let _guard = CALLBACK_TEST_GUARD.lock();
    CALLBACK_UNREGISTRATIONS.lock().clear();

    let subscriber = super::ExportedSubscriber::new(sdk_external_api_test_subscriber);
    unsafe {
        subscriber.subscribe(77);
    }

    assert_eq!(CALLBACK_UNREGISTRATIONS.lock().as_slice(), &[77]);
}

#[test]
fn export_plugin_api_macro_returns_versioned_tables() {
    assert_eq!(
        sdk_external_api_test_request(1).cast::<TestApi>(),
        &API_V1 as *const _ as *mut _
    );
    assert_eq!(
        sdk_external_api_test_request(2).cast::<TestApi>(),
        &API_V2 as *const _ as *mut _
    );
    assert!(sdk_external_api_test_request(3).is_null());
    assert_eq!(
        sdk_external_api_test_custom_request(7).cast::<TestApi>(),
        &API_V2 as *const _ as *mut _
    );
    assert!(sdk_external_api_test_custom_request(8).is_null());
}

#[test]
fn export_plugin_symbol_macro_returns_named_tables() {
    assert_eq!(
        sdk_external_api_test_symbol().cast::<TestApi>(),
        &API_V1 as *const _ as *mut _
    );
    assert_eq!(
        sdk_external_api_test_custom_symbol().cast::<TestApi>(),
        &API_V2 as *const _ as *mut _
    );
    assert!(sdk_external_api_test_missing_symbol().is_null());
}

#[test]
fn define_plugin_symbols_macro_loads_required_and_optional_exports() {
    let symbols = unsafe { Kernel32ExternalApiFlatSymbols::load_dll("kernel32.dll") }
        .expect("expected to resolve test exports from kernel32.dll");

    assert_ne!(unsafe { (symbols.current_process_id)() }, 0);
    assert_ne!(unsafe { (symbols.current_thread_id)() }, 0);
    assert!(symbols.maybe_missing_export.is_none());
}

#[test]
fn optional_symbol_helpers_probe_flat_capabilities() {
    let current_process_id = unsafe {
        super::symbol_optional::<unsafe extern "system" fn() -> u32>(
            "kernel32.dll",
            c"GetCurrentProcessId",
        )
    }
    .expect("kernel32.dll should be loaded")
    .expect("expected GetCurrentProcessId export");
    assert_ne!(unsafe { current_process_id() }, 0);

    let missing = unsafe {
        super::symbol_optional::<unsafe extern "system" fn() -> u32>(
            "kernel32.dll",
            c"SdkExternalApiDefinitelyMissing",
        )
    }
    .expect("kernel32.dll should be loaded");
    assert!(missing.is_none());

    let process_handle = unsafe {
        super::request_service_optional::<core::ffi::c_void>("kernel32.dll", c"GetCurrentProcess")
    }
    .expect("kernel32.dll should be loaded");
    assert!(process_handle.is_some());

    let missing_service = unsafe {
        super::request_service_optional::<core::ffi::c_void>(
            "kernel32.dll",
            c"SdkExternalApiDefinitelyMissing",
        )
    }
    .expect("kernel32.dll should be loaded");
    assert!(missing_service.is_none());
}

#[test]
fn interface_loader_request_is_decoded_from_message() {
    let mut request = InterfaceLoaderRequest::new(7u32);
    let mut command = InterfaceLoaderCommand::request(0x9007_CA50, &mut request);
    let message = Message {
        sender: c"SmoothCam".as_ptr(),
        msg_type: 0,
        data_len: core::mem::size_of::<InterfaceLoaderCommand>() as u32,
        data: (&mut command as *mut InterfaceLoaderCommand).cast(),
    };

    let decoded = unsafe {
        interface_loader_request_from_message_mut::<u32>(MessageRef::new(&message), 0x9007_CA50)
    }
    .expect("expected to decode the nested request payload");
    assert_eq!(decoded.version, 7);
}

#[test]
fn interface_loader_request_rejects_wrong_header() {
    let mut request = InterfaceLoaderRequest::new(3u16);
    let mut command = InterfaceLoaderCommand::request(0xDEAD_BEEF, &mut request);
    let message = Message {
        sender: c"SmoothCam".as_ptr(),
        msg_type: 0,
        data_len: core::mem::size_of::<InterfaceLoaderCommand>() as u32,
        data: (&mut command as *mut InterfaceLoaderCommand).cast(),
    };

    let decoded = unsafe {
        interface_loader_request_from_message_mut::<u16>(MessageRef::new(&message), 0x9007_CA50)
    };
    assert!(decoded.is_none());
}

#[test]
fn interface_loader_response_decodes_provider_payload() {
    let mut provider = InterfaceProvider::new(2u32, &API_V2);
    let mut response = InterfaceLoaderResponse::provider(&mut provider);
    let message = Message {
        sender: c"TrueHUD".as_ptr(),
        msg_type: 0,
        data_len: core::mem::size_of::<InterfaceLoaderResponse>() as u32,
        data: (&mut response as *mut InterfaceLoaderResponse).cast(),
    };

    let decoded =
        unsafe { interface_loader_response_from_message::<u32>(MessageRef::new(&message)) }
            .expect("expected to decode one provider response");
    match decoded {
        InterfaceLoaderResponsePayload::Error => panic!("unexpected error response"),
        InterfaceLoaderResponsePayload::InterfaceProvider(provider) => {
            assert_eq!(provider.version, 2);
            assert_eq!(
                provider
                    .cast::<TestApi>()
                    .expect("provider must point to API_V2")
                    .as_ptr(),
                &API_V2 as *const _ as *mut _
            );
        }
    }
}

#[test]
fn interface_loader_response_decodes_error_payload() {
    let mut response = InterfaceLoaderResponse::error();
    let message = Message {
        sender: c"SmoothCam".as_ptr(),
        msg_type: 0,
        data_len: core::mem::size_of::<InterfaceLoaderResponse>() as u32,
        data: (&mut response as *mut InterfaceLoaderResponse).cast(),
    };

    assert_eq!(
        unsafe { interface_loader_response_from_message::<u16>(MessageRef::new(&message)) },
        Some(InterfaceLoaderResponsePayload::Error)
    );
    assert_eq!(response.kind, InterfaceLoaderResponseKind::Error);
}

#[test]
fn reply_interface_loader_error_requires_sender_name() {
    let message = Message {
        sender: core::ptr::null(),
        msg_type: 0,
        data_len: 0,
        data: core::ptr::null_mut(),
    };

    assert!(matches!(
        reply_interface_loader_error(MessageRef::new(&message), 0),
        Err(super::InterfaceLoaderReplyError::MissingSender)
    ));
}
