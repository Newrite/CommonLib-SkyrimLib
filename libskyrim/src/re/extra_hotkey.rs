use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraHotkey;
use crate::offsets::offsets_vtable::VTABLE_ExtraHotkey;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraHotkey::Hotkey`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hotkey {
    Unbound = -1,
    Slot1 = 0,
    Slot2 = 1,
    Slot3 = 2,
    Slot4 = 3,
    Slot5 = 4,
    Slot6 = 5,
    Slot7 = 6,
    Slot8 = 7,
}

core_util::impl_enumset_type!(Hotkey => u8);

impl TryFrom<u8> for Hotkey {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xFF => Ok(Self::Unbound),
            0..=7 => Ok(unsafe { core::mem::transmute::<i32, Self>(i32::from(value)) }),
            _ => Err(()),
        }
    }
}

/// C++ `RE::ExtraHotkey`
#[repr(C)]
pub struct ExtraHotkey {
    pub base: BSExtraData,           // 00
    pub hotkey: EnumSet<Hotkey, u8>, // 10
    pub unk11: u8,                   // 11
    pub unk12: u16,                  // 12
    pub unk14: u32,                  // 14
}

const _: () = assert!(core::mem::size_of::<ExtraHotkey>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraHotkey, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraHotkey, hotkey) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraHotkey, unk11) == 0x11);
const _: () = assert!(core::mem::offset_of!(ExtraHotkey, unk12) == 0x12);
const _: () = assert!(core::mem::offset_of!(ExtraHotkey, unk14) == 0x14);

impl RttiType for ExtraHotkey {
    const RTTI: VariantID = RTTI_ExtraHotkey;
}

impl ExtraDataTyped for ExtraHotkey {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Hotkey;
}

inherit!(ExtraHotkey : BSExtraData);

impl ExtraHotkey {
    pub const RTTI: VariantID = RTTI_ExtraHotkey;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraHotkey;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Hotkey;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kHotkey; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(hotkey: Hotkey) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            hotkey: EnumSet::from_underlying(hotkey as u8),
            unk11: 0,
            unk12: 0,
            unk14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.hotkey != unsafe { (*rhs).hotkey }
    }
}
