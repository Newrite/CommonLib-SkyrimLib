#![allow(non_snake_case)]

pub mod RaceSexMenuEvent {
    /// C++ `RE::RaceSexMenuEvent::NameChangedEvent`
    #[repr(C)]
    pub struct NameChangedEvent {
        pub _data: u8,
    }

    const _: () = assert!(core::mem::size_of::<NameChangedEvent>() == 0x1);
}
