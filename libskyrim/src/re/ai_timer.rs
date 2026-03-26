/// C++ `RE::AITimer`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AITimer {
    pub ai_timer: f32, // 00
    pub timer: f32,    // 04
}

const _: () = assert!(core::mem::size_of::<AITimer>() == 0x08);
const _: () = assert!(core::mem::offset_of!(AITimer, ai_timer) == 0x00);
const _: () = assert!(core::mem::offset_of!(AITimer, timer) == 0x04);
