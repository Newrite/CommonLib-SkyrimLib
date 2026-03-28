use core::ffi::c_char;

use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESIdleForm;
use crate::offsets::offsets_vtable::VTABLE_TESIdleForm;
use crate::re::actor::Actor;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_string::BSString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::ni_form_array::NiFormArray;
use crate::re::tes_condition::TESCondition;
use crate::re::tes_form::TESForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::IDLE_DATA::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESIdleFormFlag {
    None = 0,
    Parent = 1 << 0,
    Sequence = 1 << 1,
    NoAttacking = 1 << 2,
    Blocking = 1 << 3,
}

core_util::impl_enumset_type!(TESIdleFormFlag => u8);

/// C++ `RE::IDLE_DATA`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDLE_DATA {
    pub loop_min: i8,                        // 00
    pub loop_max: i8,                        // 01
    pub flags: EnumSet<TESIdleFormFlag, u8>, // 02
    pub animation_group_selection: u8,       // 03
    pub replay_delay: u16,                   // 04
}

const _: () = assert!(core::mem::size_of::<IDLE_DATA>() == 0x6);
const _: () = assert!(core::mem::offset_of!(IDLE_DATA, loop_min) == 0x00);
const _: () = assert!(core::mem::offset_of!(IDLE_DATA, loop_max) == 0x01);
const _: () = assert!(core::mem::offset_of!(IDLE_DATA, flags) == 0x02);
const _: () = assert!(core::mem::offset_of!(IDLE_DATA, animation_group_selection) == 0x03);
const _: () = assert!(core::mem::offset_of!(IDLE_DATA, replay_delay) == 0x04);

bitflags! {
    /// C++ `RE::TESIdleForm::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESIdleFormRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESIdleForm`
#[repr(C)]
pub struct TESIdleForm {
    pub base: TESForm,                  // 00
    pub conditions: TESCondition,       // 20
    pub data: IDLE_DATA,                // 28 - DATA
    pub pad2e: u16,                     // 2E
    pub child_idles: *mut NiFormArray,  // 30
    pub parent_idle: *mut TESIdleForm,  // 38 - ANAM~
    pub prev_idle: *mut TESIdleForm,    // 40 - ~ANAM
    pub anim_file_name: BSFixedString,  // 48 - DNAM
    pub anim_event_name: BSFixedString, // 50 - ENAM
    pub form_editor_id: BSString,       // 58 - EDID
}

const _: () = assert!(core::mem::size_of::<TESIdleForm>() == 0x68);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, conditions) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, data) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, child_idles) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, parent_idle) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, prev_idle) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, anim_file_name) == 0x48);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, anim_event_name) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESIdleForm, form_editor_id) == 0x58);

impl RttiType for TESIdleForm {
    const RTTI: VariantID = RTTI_TESIdleForm;
}

impl FormCastable for TESIdleForm {
    const TARGET_FORM_TYPE: FormType = FormType::Idle;
}

inherit!(TESIdleForm : TESForm);

impl TESIdleForm {
    pub const RTTI: VariantID = RTTI_TESIdleForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESIdleForm;
    pub const FORMTYPE: FormType = FormType::Idle;

    // override (TESForm)
    // void        InitializeData() override;                                                                          // 04
    // void        ClearData() override;                                                                               // 05
    // bool        Load(TESFile* a_mod) override;                                                                      // 06
    // TESForm*    CreateDuplicateForm(bool a_createEditorID, NiTPointerMap<TESForm*, TESForm*>* a_copyMap) override;  // 09
    // void        InitItemImpl() override;                                                                            // 13
    // const char* GetFormEditorID() const override;                                                                   // 32
    // bool        SetFormEditorID(const char* a_str) override;                                                        // 33

    crate::relocation_func! {
        pub fn check_conditions(
            &mut self,
            actor: *mut Actor,
            target: *mut TESObjectREFR,
            check_parent_idle: bool
        ) -> bool => RelocationID::new(24069, 24572)
    }

    #[inline(always)]
    pub fn get_form_editor_id_local(&self) -> *const c_char {
        self.form_editor_id.c_str()
    }

    #[inline(always)]
    pub fn get_form_editor_id_local_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_form_editor_id_local())
    }

    #[inline(always)]
    pub fn set_form_editor_id_local(&mut self, editor_id: *const c_char) -> bool {
        self.form_editor_id.set_c_str(editor_id, 0)
    }
}
