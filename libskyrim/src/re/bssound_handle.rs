use core_util::EnumSet;

/// C++ `RE::BSSoundHandle::AssumedState`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSSoundHandleAssumedState {
    Initialized = 0,
    Playing = 1,
    Stopped = 2,
    Paused = 3,
}

core_util::impl_enumset_type!(BSSoundHandleAssumedState => u32);

/// C++ `RE::BSSoundHandle::LoopType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSSoundHandleLoopType {
    None = 0,
    WholeFile = 1,
    EnvFast = 2,
    EnvSlow = 3,
}

/// C++ `RE::BSSoundHandle`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSSoundHandle {
    pub sound_id: u32,                                  // 00
    pub assume_success: bool,                           // 04
    pub pad05: u8,                                      // 05
    pub pad06: u16,                                     // 06
    pub state: EnumSet<BSSoundHandleAssumedState, u32>, // 08
}

const _: () = assert!(core::mem::size_of::<BSSoundHandle>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSSoundHandle, sound_id) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSSoundHandle, assume_success) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSSoundHandle, state) == 0x08);

impl BSSoundHandle {
    pub const INVALID_ID: u32 = u32::MAX;

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.sound_id != Self::INVALID_ID
    }

    #[inline(always)]
    pub fn is_playing(&self) -> bool {
        self.state == EnumSet::from_underlying(BSSoundHandleAssumedState::Playing as u32)
    }
}
