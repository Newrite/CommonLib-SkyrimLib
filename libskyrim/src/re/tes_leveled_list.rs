use alloc::vec::Vec;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESLeveledList;
use crate::offsets::offsets_vtable::VTABLE_TESLeveledList;
use crate::re::BSScrapArray;
use crate::re::ContainerItemExtra;
use crate::re::FormType;
use crate::re::SimpleArray;
use crate::re::TESForm;
use crate::re::TESGlobal;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RelocationID, RttiType, VariantID, skyrim_cast};
use crate::virtual_method;

#[repr(C)]
pub struct LeveledObject {
    pub form: *mut TESForm,                  // 00
    pub count: u16,                          // 08
    pub level: u16,                          // 0A
    pub pad0c: u32,                          // 0C
    pub item_extra: *mut ContainerItemExtra, // 10
}

const _: () = assert!(core::mem::size_of::<LeveledObject>() == 0x18);
const _: () = assert!(core::mem::offset_of!(LeveledObject, form) == 0x00);
const _: () = assert!(core::mem::offset_of!(LeveledObject, count) == 0x08);
const _: () = assert!(core::mem::offset_of!(LeveledObject, item_extra) == 0x10);

#[repr(C)]
pub struct CalcedObject {
    pub form: *mut TESForm,                 // 00
    pub count: u16,                         // 08
    pub pad0a: u16,                         // 0A
    pub pad0c: u32,                         // 0C
    pub container_item: ContainerItemExtra, // 10
}

const _: () = assert!(core::mem::size_of::<CalcedObject>() == 0x28);
const _: () = assert!(core::mem::offset_of!(CalcedObject, form) == 0x00);
const _: () = assert!(core::mem::offset_of!(CalcedObject, container_item) == 0x10);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESLeveledListFlags: u8 {
        const CALCULATE_FROM_ALL_LEVELS_LTE_PC_LEVEL = 1 << 0;
        const CALCULATE_FOR_EACH_ITEM_IN_COUNT = 1 << 1;
        const USE_ALL = 1 << 2;
        const SPECIAL_LOOT = 1 << 3;
    }
}

#[repr(C)]
pub struct TESLeveledList {
    pub base: BaseFormComponent,             // 00
    pub entries: SimpleArray<LeveledObject>, // 08
    pub chance_none: i8,                     // 10
    pub ll_flags: TESLeveledListFlags,       // 11
    pub num_entries: u8,                     // 12
    pub unk13: u8,                           // 13
    pub pad14: u32,                          // 14
    pub unk18: *mut core::ffi::c_void,       // 18
    pub chance_global: *mut TESGlobal,       // 20
}

const _: () = assert!(core::mem::size_of::<TESLeveledList>() == 0x28);
const _: () = assert!(core::mem::offset_of!(TESLeveledList, entries) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESLeveledList, chance_none) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESLeveledList, chance_global) == 0x20);

impl RttiType for TESLeveledList {
    const RTTI: VariantID = RTTI_TESLeveledList;
}

impl AsRef<TESLeveledList> for TESLeveledList {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESLeveledList> for TESLeveledList {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESLeveledList : BaseFormComponent);

impl TESLeveledList {
    pub const RTTI: VariantID = RTTI_TESLeveledList;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESLeveledList;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    // add
    // virtual u8   GetChanceNone();                      // 04
    // virtual bool GetMultCalc();                        // 05
    // virtual i32  GetLevDifferenceMax();                // 06
    // virtual bool GetCanContainFormsOfType(...) const;  // 07

    virtual_method! {
        pub const VFUNC_GET_CHANCE_NONE: usize = 0x04;
        pub fn get_chance_none() -> u8
    }

    virtual_method! {
        pub const VFUNC_GET_MULT_CALC: usize = 0x05;
        pub fn get_mult_calc() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_LEV_DIFFERENCE_MAX: usize = 0x06;
        pub fn get_lev_difference_max() -> i32
    }

    virtual_method! {
        pub const VFUNC_GET_CAN_CONTAIN_FORMS_OF_TYPE: usize = 0x07;
        pub fn get_can_contain_forms_of_type(form_type: FormType) -> bool
    }

    crate::relocation_func! {
        pub fn calculate_current_form_list(
            &mut self,
            level: u16,
            count: i16,
            calced_objects: &mut BSScrapArray<CalcedObject>,
            arg5: u32,
            use_player_level: bool
        ) => RelocationID::new(14579, 14751)
    }

    #[inline]
    pub fn entries_slice(&self) -> &[LeveledObject] {
        self.entries.as_slice()
    }

    pub fn get_contained_forms(&self) -> Vec<*mut TESForm> {
        let mut results = Vec::new();
        let mut queued = Vec::new();
        queued.push(self as *const TESLeveledList);

        while let Some(current_ptr) = queued.pop() {
            let current = unsafe { &*current_ptr };
            for entry in current.entries_slice() {
                let form = entry.form;
                if form.is_null() {
                    continue;
                }

                let nested = unsafe { skyrim_cast::<TESForm, TESLeveledList>(form) };
                if !nested.is_null() {
                    queued.push(nested.cast_const());
                } else {
                    results.push(form);
                }
            }
        }

        results
    }
}

pub trait TESLeveledListExt {
    fn get_chance_none(&self) -> u8;
    fn get_mult_calc(&self) -> bool;
    fn get_lev_difference_max(&self) -> i32;
    fn get_can_contain_forms_of_type(&self, form_type: FormType) -> bool;
    fn calculate_current_form_list(
        &mut self,
        level: u16,
        count: i16,
        calced_objects: &mut BSScrapArray<CalcedObject>,
        arg5: u32,
        use_player_level: bool,
    );
    fn entries_slice(&self) -> &[LeveledObject];
    fn get_contained_forms(&self) -> Vec<*mut TESForm>;
}

impl<T: AsRef<TESLeveledList> + AsMut<TESLeveledList>> TESLeveledListExt for T {
    #[inline(always)]
    fn get_chance_none(&self) -> u8 {
        self.as_ref().get_chance_none()
    }

    #[inline(always)]
    fn get_mult_calc(&self) -> bool {
        self.as_ref().get_mult_calc()
    }

    #[inline(always)]
    fn get_lev_difference_max(&self) -> i32 {
        self.as_ref().get_lev_difference_max()
    }

    #[inline(always)]
    fn get_can_contain_forms_of_type(&self, form_type: FormType) -> bool {
        self.as_ref().get_can_contain_forms_of_type(form_type)
    }

    #[inline(always)]
    fn calculate_current_form_list(
        &mut self,
        level: u16,
        count: i16,
        calced_objects: &mut BSScrapArray<CalcedObject>,
        arg5: u32,
        use_player_level: bool,
    ) {
        TESLeveledList::calculate_current_form_list(
            self.as_mut(),
            level,
            count,
            calced_objects,
            arg5,
            use_player_level,
        )
    }

    #[inline(always)]
    fn entries_slice(&self) -> &[LeveledObject] {
        self.as_ref().entries_slice()
    }

    #[inline(always)]
    fn get_contained_forms(&self) -> Vec<*mut TESForm> {
        self.as_ref().get_contained_forms()
    }
}
