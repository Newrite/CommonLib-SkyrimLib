use crate::offsets::offsets_rtti::RTTI_TESEnchantableForm;
use crate::offsets::offsets_vtable::VTABLE_TESEnchantableForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::enchantment_item::EnchantmentItem;
use crate::re::magic_system::CastingType;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::{Enum, inherit};

#[repr(C)]
pub struct TESEnchantableForm {
    pub base: BaseFormComponent,               // 00
    pub form_enchanting: *mut EnchantmentItem, // 08 - EITM
    pub casting_type: u16,                     // 10
    pub amount_of_enchantment: u16,            // 12 - EAMT
    pub pad14: u32,                            // 14
}

const _: () = assert!(core::mem::size_of::<TESEnchantableForm>() == 0x18);

impl RttiType for TESEnchantableForm {
    const RTTI: VariantID = RTTI_TESEnchantableForm;
}

inherit!(TESEnchantableForm : BaseFormComponent);

impl TESEnchantableForm {
    pub const RTTI: VariantID = RTTI_TESEnchantableForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESEnchantableForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    virtual_method! {
        pub const GET_CASTING_TYPE: usize = 0x04;
        pub fn get_casting_type() -> CastingType
    }

    #[inline(always)]
    pub const fn casting_type_storage(&self) -> Enum<CastingType, u16> {
        Enum::from_underlying(self.casting_type)
    }

    #[inline(always)]
    pub fn try_casting_type(&self) -> Option<CastingType> {
        self.casting_type_storage().get()
    }

    #[inline]
    pub fn get_enchantment(&self) -> Option<&EnchantmentItem> {
        // SAFETY: form_enchanting is a pointer to an engine-owned object or null
        unsafe { self.form_enchanting.as_ref() }
    }
}

pub trait TESEnchantableFormExt {
    fn casting_type_storage(&self) -> Enum<CastingType, u16>;
    fn get_enchantment(&self) -> Option<&EnchantmentItem>;
    fn get_casting_type(&self) -> CastingType;
    fn try_casting_type(&self) -> Option<CastingType>;
}

impl<T: AsRef<TESEnchantableForm>> TESEnchantableFormExt for T {
    fn casting_type_storage(&self) -> Enum<CastingType, u16> {
        self.as_ref().casting_type_storage()
    }

    fn get_enchantment(&self) -> Option<&EnchantmentItem> {
        self.as_ref().get_enchantment()
    }

    fn get_casting_type(&self) -> CastingType {
        self.as_ref().get_casting_type()
    }

    fn try_casting_type(&self) -> Option<CastingType> {
        self.as_ref().try_casting_type()
    }
}
