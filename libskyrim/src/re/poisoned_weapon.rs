#![allow(non_snake_case)]

pub mod PoisonedWeapon {
    /// C++ `RE::PoisonedWeapon::Event`
    #[repr(C)]
    pub struct Event {
        pub _data: u8,
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x1);
}
