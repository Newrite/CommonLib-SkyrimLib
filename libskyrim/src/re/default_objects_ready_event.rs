#![allow(non_snake_case)]

pub mod DefaultObjectsReadyEvent {
    /// C++ `RE::DefaultObjectsReadyEvent::Event`
    #[repr(C)]
    pub struct Event {
        pub _data: u8,
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x1);
}
