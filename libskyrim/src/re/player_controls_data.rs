use crate::re::NiPoint2;

/// C++ `RE::PlayerControlsData`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerControlsData {
    pub move_input_vec: NiPoint2, // 00
    pub look_input_vec: NiPoint2, // 08
    pub prev_move_vec: NiPoint2,  // 10
    pub prev_look_vec: NiPoint2,  // 18
    pub unk20: u32,               // 20
    pub auto_move: bool,          // 24
    pub running: bool,            // 25
    pub unk26: u8,                // 26
    pub fov_slide_mode: bool,     // 27
    pub pov_script_mode: bool,    // 28
    pub pov_beast_mode: bool,     // 29
    pub unk2a: u8,                // 2A
    pub unk2b: u8,                // 2B
    pub remap_mode: bool,         // 2C
    pub unk2d: u8,                // 2D
    pub unk2e: u16,               // 2E
}

const _: () = assert!(core::mem::size_of::<PlayerControlsData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, move_input_vec) == 0x00);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, look_input_vec) == 0x08);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, prev_move_vec) == 0x10);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, prev_look_vec) == 0x18);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, unk20) == 0x20);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, auto_move) == 0x24);
const _: () = assert!(core::mem::offset_of!(PlayerControlsData, remap_mode) == 0x2C);
