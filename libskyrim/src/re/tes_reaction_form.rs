use core_util::EnumSet;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESReactionForm;
use crate::offsets::offsets_vtable::VTABLE_TESReactionForm;
use crate::re::BSSimpleList;
use crate::re::FIGHT_REACTION;
use crate::re::FormType;
use crate::re::TESForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct GroupReaction {
    pub form: *mut TESForm,             // 00
    pub reaction: i32,                  // 08
    pub fight_reaction: FIGHT_REACTION, // 0C
}

const _: () = assert!(core::mem::size_of::<GroupReaction>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GroupReaction, form) == 0x00);
const _: () = assert!(core::mem::offset_of!(GroupReaction, reaction) == 0x08);
const _: () = assert!(core::mem::offset_of!(GroupReaction, fight_reaction) == 0x0C);

#[repr(C)]
pub struct TESReactionForm {
    pub base: BaseFormComponent,                     // 00
    pub reactions: BSSimpleList<*mut GroupReaction>, // 08
    pub group_form_type: EnumSet<FormType, u8>,      // 18
    pub pad19: u8,                                   // 19
    pub pad1a: u16,                                  // 1A
    pub pad1c: u32,                                  // 1C
}

const _: () = assert!(core::mem::size_of::<TESReactionForm>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESReactionForm, reactions) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESReactionForm, group_form_type) == 0x18);

impl RttiType for TESReactionForm {
    const RTTI: VariantID = RTTI_TESReactionForm;
}

inherit!(TESReactionForm : BaseFormComponent);

impl TESReactionForm {
    pub const RTTI: VariantID = RTTI_TESReactionForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESReactionForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
