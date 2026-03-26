#![allow(non_camel_case_types)]

use crate::re::BSFixedString;

#[allow(non_snake_case)]
pub mod Movement {
    use super::BSFixedString;

    /// C++ `RE::Movement::SPEED_DIRECTIONS::SPEED_DIRECTION`
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SPEED_DIRECTION {
        Left = 0,
        Right = 1,
        Forward = 2,
        Back = 3,
        Rotations = 4,
        Total = 5,
    }

    /// C++ `RE::Movement::MaxSpeeds`
    #[repr(C)]
    pub struct MaxSpeeds {
        pub speeds: [[f32; 2]; 5],        // 00
        pub rotate_while_moving_run: f32, // 28
    }

    const _: () = assert!(core::mem::size_of::<MaxSpeeds>() == 0x2C);
    const _: () = assert!(core::mem::offset_of!(MaxSpeeds, speeds) == 0x00);
    const _: () = assert!(core::mem::offset_of!(MaxSpeeds, rotate_while_moving_run) == 0x28);

    /// C++ `RE::Movement::TypeData`
    #[repr(C)]
    pub struct TypeData {
        pub type_name: BSFixedString, // 00
        pub default_data: MaxSpeeds,  // 08
        pub directional: f32,         // 34
        pub movement_speed: f32,      // 38
        pub rotation_speed: f32,      // 3C
    }

    const _: () = assert!(core::mem::size_of::<TypeData>() == 0x40);
    const _: () = assert!(core::mem::offset_of!(TypeData, type_name) == 0x00);
    const _: () = assert!(core::mem::offset_of!(TypeData, default_data) == 0x08);
    const _: () = assert!(core::mem::offset_of!(TypeData, directional) == 0x34);
    const _: () = assert!(core::mem::offset_of!(TypeData, movement_speed) == 0x38);
    const _: () = assert!(core::mem::offset_of!(TypeData, rotation_speed) == 0x3C);
}
