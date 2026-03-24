use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraEnchantment;
use crate::offsets::offsets_vtable::VTABLE_ExtraEnchantment;
use crate::re::{BSExtraData, EnchantmentItem, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraEnchantment`
#[repr(C)]
pub struct ExtraEnchantment {
    pub base: BSExtraData,                 // 00
    pub enchantment: *mut EnchantmentItem, // 10
    pub charge: u16,                       // 18
    pub remove_on_unequip: bool,           // 1A
    pub pad1b: u8,                         // 1B
    pub pad1c: u32,                        // 1C
}

const _: () = assert!(core::mem::size_of::<ExtraEnchantment>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, enchantment) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, charge) == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, remove_on_unequip) == 0x1A);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, pad1b) == 0x1B);
const _: () = assert!(core::mem::offset_of!(ExtraEnchantment, pad1c) == 0x1C);

impl RttiType for ExtraEnchantment {
    const RTTI: VariantID = RTTI_ExtraEnchantment;
}

impl ExtraDataTyped for ExtraEnchantment {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Enchantment;
}

inherit!(ExtraEnchantment : BSExtraData);

impl ExtraEnchantment {
    pub const RTTI: VariantID = RTTI_ExtraEnchantment;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraEnchantment;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Enchantment;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kEnchantment; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(enchantment: *mut EnchantmentItem, charge: u16, remove_on_unequip: bool) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            enchantment,
            charge,
            remove_on_unequip,
            pad1b: 0,
            pad1c: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null()
            || self.enchantment != unsafe { (*rhs).enchantment }
            || self.charge != unsafe { (*rhs).charge }
            || self.remove_on_unequip != unsafe { (*rhs).remove_on_unequip }
    }
}
