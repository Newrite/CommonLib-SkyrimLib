use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESActorBase;
use crate::offsets::offsets_vtable::VTABLE_TESActorBase;
use crate::re::ActorValueOwner;
use crate::re::BGSAttackDataForm;
use crate::re::BGSDestructibleObjectForm;
use crate::re::BGSKeywordForm;
use crate::re::BGSPerkRankArray;
use crate::re::BGSSkinForm;
use crate::re::FormType;
use crate::re::TESAIForm;
use crate::re::TESActorBaseData;
use crate::re::TESBoundAnimObject;
use crate::re::TESCombatStyle;
use crate::re::TESContainer;
use crate::re::TESForm;
use crate::re::TESFullName;
use crate::re::TESSpellList;
use crate::relocation::{RttiType, VariantID, skyrim_cast};
use crate::virtual_method;

#[repr(C)]
pub struct TESActorBase {
    pub base: TESBoundAnimObject,                            // 000
    pub actor_base_data: TESActorBaseData,                   // 030
    pub container: TESContainer,                             // 088
    pub spell_list: TESSpellList,                            // 0A0
    pub ai_form: TESAIForm,                                  // 0B0
    pub full_name: TESFullName,                              // 0D8
    pub actor_value_owner: ActorValueOwner,                  // 0E8
    pub destructible_object_form: BGSDestructibleObjectForm, // 0F0
    pub skin_form: BGSSkinForm,                              // 100
    pub keyword_form: BGSKeywordForm,                        // 110
    pub attack_data_form: BGSAttackDataForm,                 // 128
    pub perk_rank_array: BGSPerkRankArray,                   // 138
}

const _: () = assert!(core::mem::size_of::<TESActorBase>() == 0x150);
const _: () = assert!(core::mem::offset_of!(TESActorBase, actor_base_data) == 0x030);
const _: () = assert!(core::mem::offset_of!(TESActorBase, container) == 0x088);
const _: () = assert!(core::mem::offset_of!(TESActorBase, spell_list) == 0x0A0);
const _: () = assert!(core::mem::offset_of!(TESActorBase, ai_form) == 0x0B0);
const _: () = assert!(core::mem::offset_of!(TESActorBase, full_name) == 0x0D8);
const _: () = assert!(core::mem::offset_of!(TESActorBase, actor_value_owner) == 0x0E8);
const _: () = assert!(core::mem::offset_of!(TESActorBase, destructible_object_form) == 0x0F0);
const _: () = assert!(core::mem::offset_of!(TESActorBase, skin_form) == 0x100);
const _: () = assert!(core::mem::offset_of!(TESActorBase, keyword_form) == 0x110);
const _: () = assert!(core::mem::offset_of!(TESActorBase, attack_data_form) == 0x128);
const _: () = assert!(core::mem::offset_of!(TESActorBase, perk_rank_array) == 0x138);

impl RttiType for TESActorBase {
    const RTTI: VariantID = RTTI_TESActorBase;
}

impl AsRef<TESActorBase> for TESActorBase {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESActorBase> for TESActorBase {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESActorBase : TESBoundAnimObject);
inherit!(TESActorBase => TESActorBaseData, actor_base_data);
inherit!(TESActorBase => TESContainer, container);
inherit!(TESActorBase => TESSpellList, spell_list);
inherit!(TESActorBase => TESAIForm, ai_form);
inherit!(TESActorBase => TESFullName, full_name);
inherit!(TESActorBase => ActorValueOwner, actor_value_owner);
inherit!(TESActorBase => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESActorBase => BGSSkinForm, skin_form);
inherit!(TESActorBase => BGSKeywordForm, keyword_form);
inherit!(TESActorBase => BGSAttackDataForm, attack_data_form);
inherit!(TESActorBase => BGSPerkRankArray, perk_rank_array);

impl TESActorBase {
    pub const RTTI: VariantID = RTTI_TESActorBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESActorBase;

    // override (TESBoundAnimObject)
    // bool AddChange(std::uint32_t a_changeFlags) override;     // 0A
    // void RemoveChange(std::uint32_t a_changeFlags) override;  // 0B
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;         // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;         // 0F
    // bool IsAutoCalc() const override;                         // 3E
    // void SetAutoCalc(bool a_autoCalc) override;               // 3F

    // override (ActorValueOwner)
    // float GetActorValue(...) const override;                  // 01
    // float GetPermanentActorValue(...) const override;         // 02
    // float GetBaseActorValue(...) const override;              // 03
    // void  SetBaseActorValue(...) override;                    // 04
    // void  ModBaseActorValue(...) override;                    // 05
    // void  ModActorValue(...) override;                        // 06
    // void  SetActorValue(...) override;                        // 07
    // bool  GetIsPlayerOwner() const override;                  // 08

    // add
    // virtual bool            GetHasPLSpecTex() const;          // 53
    // virtual TESCombatStyle* GetCombatStyle();                 // 54
    // virtual void            SetCombatStyle(...);              // 55
    // virtual TESForm*        GetAsForm();                      // 56

    virtual_method! {
        pub const VFUNC_GET_HAS_PL_SPEC_TEX: usize = 0x53;
        pub fn get_has_pl_spec_tex() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_COMBAT_STYLE: usize = 0x54;
        pub fn get_combat_style() -> *mut TESCombatStyle
    }

    virtual_method! {
        pub const VFUNC_SET_COMBAT_STYLE: usize = 0x55;
        pub fn set_combat_style(combat_style: *mut TESCombatStyle)
    }

    virtual_method! {
        pub const VFUNC_GET_AS_FORM: usize = 0x56;
        pub fn get_as_form() -> *mut TESForm
    }

    pub fn is_leveled(&self) -> bool {
        let mut base = unsafe {
            skyrim_cast::<TESForm, TESActorBase>(self.actor_base_data.base_template_form)
        };
        while !base.is_null() {
            let current = unsafe { &*base };
            if current.base.base.base.base.get_form_type() == FormType::LeveledNPC {
                return true;
            }
            base = unsafe {
                skyrim_cast::<TESForm, TESActorBase>(current.actor_base_data.base_template_form)
            };
        }
        false
    }
}

pub trait TESActorBaseExt {
    fn get_has_pl_spec_tex(&self) -> bool;
    fn get_combat_style(&self) -> *mut TESCombatStyle;
    fn set_combat_style(&self, combat_style: *mut TESCombatStyle);
    fn get_as_form(&self) -> *mut TESForm;
    fn is_leveled(&self) -> bool;
}

impl<T: AsRef<TESActorBase>> TESActorBaseExt for T {
    fn get_has_pl_spec_tex(&self) -> bool {
        self.as_ref().get_has_pl_spec_tex()
    }

    fn get_combat_style(&self) -> *mut TESCombatStyle {
        self.as_ref().get_combat_style()
    }

    fn set_combat_style(&self, combat_style: *mut TESCombatStyle) {
        self.as_ref().set_combat_style(combat_style)
    }

    fn get_as_form(&self) -> *mut TESForm {
        self.as_ref().get_as_form()
    }

    fn is_leveled(&self) -> bool {
        self.as_ref().is_leveled()
    }
}
