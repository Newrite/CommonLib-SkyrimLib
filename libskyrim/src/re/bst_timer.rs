use crate::relocation::RelocationID;

/// Source-backed partial translation of `RE::BSTimer`.
#[repr(C)]
pub struct BSTimer {
    pub unk00: u64,                  // 00
    pub last_performance_count: u32, // 08
    // TODO: `BSTimer` field offsets diverge after `0x0C` because flat runtimes
    // keep `pad0C` and VR-only builds do not. Keep the shared `0x40` storage
    // tail here until a dedicated runtime-layout pass needs direct field access.
    pub runtime_tail: [u8; 0x34], // 0C
}

const _: () = assert!(core::mem::size_of::<BSTimer>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BSTimer, unk00) == 0x0);
const _: () = assert!(core::mem::offset_of!(BSTimer, last_performance_count) == 0x8);
const _: () = assert!(core::mem::offset_of!(BSTimer, runtime_tail) == 0xC);

impl BSTimer {
    crate::relocation_variable! {
        fn singleton() -> *mut BSTimer => RelocationID::new(523657, 410196), is_ptr
    }

    crate::relocation_variable! {
        fn global_time_multiplier_ptr() -> &'static f32 => RelocationID::new(511882, 388442)
    }

    crate::relocation_variable! {
        fn global_time_multiplier_target_ptr() -> &'static f32 => RelocationID::new(511883, 388443)
    }

    crate::relocation_func! {
        pub fn set_global_time_multiplier(&mut self, multiplier: f32, arg2: bool)
            => RelocationID::new(66988, 68245)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut BSTimer {
        Self::singleton()
    }

    #[inline(always)]
    pub fn q_global_time_multiplier() -> f32 {
        *Self::global_time_multiplier_ptr()
    }

    #[inline(always)]
    pub fn q_global_time_multiplier_target() -> f32 {
        *Self::global_time_multiplier_target_ptr()
    }
}
