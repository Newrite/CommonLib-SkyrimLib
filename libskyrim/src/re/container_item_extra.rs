use crate::re::TESForm;

/// C++ `RE::ContainerItemExtra::Conditional`
#[repr(C)]
pub union ContainerItemExtraConditional {
    pub global: *mut TESForm,
    pub rank: i32,
    pub pad: u64,
}

const _: () = assert!(core::mem::size_of::<ContainerItemExtraConditional>() == 0x8);

impl Default for ContainerItemExtraConditional {
    fn default() -> Self {
        Self {
            global: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::ContainerItemExtra`
#[repr(C)]
pub struct ContainerItemExtra {
    pub owner: *mut TESForm,                        // 00
    pub conditional: ContainerItemExtraConditional, // 08
    pub health_mult: f32,                           // 10
    pub pad14: u32,                                 // 14
}

const _: () = assert!(core::mem::size_of::<ContainerItemExtra>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ContainerItemExtra, owner) == 0x00);
const _: () = assert!(core::mem::offset_of!(ContainerItemExtra, conditional) == 0x08);
const _: () = assert!(core::mem::offset_of!(ContainerItemExtra, health_mult) == 0x10);

impl Default for ContainerItemExtra {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerItemExtra {
    #[inline]
    pub const fn new() -> Self {
        Self::new_with_owner(core::ptr::null_mut())
    }

    #[inline]
    pub const fn new_with_owner(owner: *mut TESForm) -> Self {
        Self {
            owner,
            conditional: ContainerItemExtraConditional {
                global: core::ptr::null_mut(),
            },
            health_mult: 100.0,
            pad14: 0,
        }
    }
}
