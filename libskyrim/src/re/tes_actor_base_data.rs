use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESActorBaseData;
use crate::offsets::offsets_vtable::VTABLE_TESActorBaseData;
use crate::re::BGSVoiceType;
use crate::re::BSTArray;
use crate::re::FACTION_RANK;
use crate::re::TESActorBase;
use crate::re::TESForm;
use crate::re::TESLevItem;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorBaseDataFlags {
    None = 0,
    Female = 1 << 0,
    Essential = 1 << 1,
    IsChargenFacePreset = 1 << 2,
    Respawn = 1 << 3,
    AutoCalcStats = 1 << 4,
    Unique = 1 << 5,
    DoesntAffectStealthMeter = 1 << 6,
    PCLevelMult = 1 << 7,
    UsesTemplate = 1 << 8,
    CalcForAllTemplates = 1 << 9,
    Protected = 1 << 11,
    NoRumors = 1 << 13,
    Summonable = 1 << 14,
    DoesntBleed = 1 << 16,
    BleedoutOverride = 1 << 18,
    OppositeGenderAnims = 1 << 19,
    SimpleActor = 1 << 20,
    LoopedScript = 1 << 21,
    NoActivation = 1 << 23,
    LoopedAudio = 1 << 28,
    IsGhost = 1 << 29,
    Invulnerable = 1 << 31,
}

core_util::impl_enumset_type!(ActorBaseDataFlags => u32);

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorBaseTemplateUseFlags {
    None = 0,
    Traits = 1 << 0,
    Stats = 1 << 1,
    Factions = 1 << 2,
    Spells = 1 << 3,
    AIData = 1 << 4,
    AIPackages = 1 << 5,
    Unused = 1 << 6,
    BaseData = 1 << 7,
    Inventory = 1 << 8,
    Script = 1 << 9,
    AIDefPackList = 1 << 10,
    AttackData = 1 << 11,
    Keywords = 1 << 12,
    CopiedTemplate = 1 << 15,
}

core_util::impl_enumset_type!(ActorBaseTemplateUseFlags => u16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ActorBaseData {
    pub actor_base_flags: EnumSet<ActorBaseDataFlags, u32>, // 00
    pub magicka_offset: i16,                                // 04
    pub stamina_offset: i16,                                // 06
    pub level: u16,                                         // 08
    pub calc_level_min: u16,                                // 0A
    pub calc_level_max: u16,                                // 0C
    pub speed_mult: u16,                                    // 0E
    pub base_disposition: u16,                              // 10
    pub template_use_flags: EnumSet<ActorBaseTemplateUseFlags, u16>, // 12
    pub health_offset: i16,                                 // 14
    pub bleedout_override: i16,                             // 16
}

const _: () = assert!(core::mem::size_of::<ActorBaseData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ActorBaseData, actor_base_flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorBaseData, template_use_flags) == 0x12);

#[repr(C)]
pub struct TESActorBaseData {
    pub base: BaseFormComponent,          // 00
    pub actor_data: ActorBaseData,        // 08
    pub death_item: *mut TESLevItem,      // 20
    pub voice_type: *mut BGSVoiceType,    // 28
    pub base_template_form: *mut TESForm, // 30
    pub change_flags: u32,                // 38
    pub pad3c: u32,                       // 3C
    pub factions: BSTArray<FACTION_RANK>, // 40
}

const _: () = assert!(core::mem::size_of::<TESActorBaseData>() == 0x58);
const _: () = assert!(core::mem::offset_of!(TESActorBaseData, actor_data) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESActorBaseData, death_item) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESActorBaseData, voice_type) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESActorBaseData, base_template_form) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESActorBaseData, factions) == 0x40);

impl RttiType for TESActorBaseData {
    const RTTI: VariantID = RTTI_TESActorBaseData;
}

impl AsRef<TESActorBaseData> for TESActorBaseData {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESActorBaseData> for TESActorBaseData {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESActorBaseData : BaseFormComponent);

impl TESActorBaseData {
    pub const RTTI: VariantID = RTTI_TESActorBaseData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESActorBaseData;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    // add
    // virtual void          CopyFromTemplateForms(TESActorBase** a_templateForms);  // 04
    // virtual bool          GetIsGhost() const;                                     // 05
    // virtual bool          GetInvulnerable() const;                                // 06
    // virtual void          Unk_07(void);                                           // 07
    // virtual BGSVoiceType* GetVoiceType(void);                                     // 08

    virtual_method! {
        pub const VFUNC_COPY_FROM_TEMPLATE_FORMS: usize = 0x04;
        pub fn copy_from_template_forms(&mut self, template_forms: *mut *mut TESActorBase)
    }

    virtual_method! {
        pub const VFUNC_GET_IS_GHOST: usize = 0x05;
        pub fn get_is_ghost() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_INVULNERABLE: usize = 0x06;
        pub fn get_invulnerable() -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07()
    }

    virtual_method! {
        pub const VFUNC_GET_VOICE_TYPE: usize = 0x08;
        pub fn get_voice_type() -> *mut BGSVoiceType
    }

    crate::relocation_func! {
        pub fn get_level(&self) -> u16 => RelocationID::new(14262, 14384)
    }

    #[inline]
    pub const fn affects_stealth_meter(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .none_underlying(ActorBaseDataFlags::DoesntAffectStealthMeter as u32)
    }

    #[inline]
    pub const fn bleeds(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .none_underlying(ActorBaseDataFlags::DoesntBleed as u32)
    }

    #[inline]
    pub const fn has_auto_calc_stats(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::AutoCalcStats as u32)
    }

    #[inline]
    pub const fn has_bleedout_override(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::BleedoutOverride as u32)
    }

    #[inline]
    pub const fn has_pc_level_mult(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::PCLevelMult as u32)
    }

    #[inline]
    pub const fn is_essential(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Essential as u32)
    }

    #[inline]
    pub const fn is_female(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Female as u32)
    }

    #[inline]
    pub fn is_ghost(&self) -> bool {
        self.get_is_ghost()
    }

    #[inline]
    pub fn is_invulnerable(&self) -> bool {
        self.get_invulnerable()
    }

    #[inline]
    pub const fn is_preset(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::IsChargenFacePreset as u32)
    }

    #[inline]
    pub const fn is_protected(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Protected as u32)
    }

    #[inline]
    pub const fn is_simple_actor(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::SimpleActor as u32)
    }

    #[inline]
    pub const fn is_summonable(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Summonable as u32)
    }

    #[inline]
    pub const fn is_unique(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Unique as u32)
    }

    #[inline]
    pub const fn respawns(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::Respawn as u32)
    }

    #[inline]
    pub const fn uses_opposite_gender_anims(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::OppositeGenderAnims as u32)
    }

    #[inline]
    pub const fn uses_template(&self) -> bool {
        self.actor_data
            .actor_base_flags
            .all_underlying(ActorBaseDataFlags::UsesTemplate as u32)
    }
}

pub trait TESActorBaseDataExt {
    fn copy_from_template_forms(&mut self, template_forms: *mut *mut TESActorBase);
    fn get_level(&self) -> u16;
    fn get_voice_type(&self) -> *mut BGSVoiceType;
    fn affects_stealth_meter(&self) -> bool;
    fn bleeds(&self) -> bool;
    fn has_auto_calc_stats(&self) -> bool;
    fn has_bleedout_override(&self) -> bool;
    fn has_pc_level_mult(&self) -> bool;
    fn is_essential(&self) -> bool;
    fn is_female(&self) -> bool;
    fn is_ghost(&self) -> bool;
    fn is_invulnerable(&self) -> bool;
    fn is_preset(&self) -> bool;
    fn is_protected(&self) -> bool;
    fn is_simple_actor(&self) -> bool;
    fn is_summonable(&self) -> bool;
    fn is_unique(&self) -> bool;
    fn respawns(&self) -> bool;
    fn uses_opposite_gender_anims(&self) -> bool;
    fn uses_template(&self) -> bool;
}

impl<T: AsRef<TESActorBaseData> + AsMut<TESActorBaseData>> TESActorBaseDataExt for T {
    fn copy_from_template_forms(&mut self, template_forms: *mut *mut TESActorBase) {
        TESActorBaseData::copy_from_template_forms(self.as_mut(), template_forms)
    }

    fn get_level(&self) -> u16 {
        TESActorBaseData::get_level(self.as_ref())
    }

    fn get_voice_type(&self) -> *mut BGSVoiceType {
        TESActorBaseData::get_voice_type(self.as_ref())
    }

    fn affects_stealth_meter(&self) -> bool {
        self.as_ref().affects_stealth_meter()
    }

    fn bleeds(&self) -> bool {
        self.as_ref().bleeds()
    }

    fn has_auto_calc_stats(&self) -> bool {
        self.as_ref().has_auto_calc_stats()
    }

    fn has_bleedout_override(&self) -> bool {
        self.as_ref().has_bleedout_override()
    }

    fn has_pc_level_mult(&self) -> bool {
        self.as_ref().has_pc_level_mult()
    }

    fn is_essential(&self) -> bool {
        self.as_ref().is_essential()
    }

    fn is_female(&self) -> bool {
        self.as_ref().is_female()
    }

    fn is_ghost(&self) -> bool {
        self.as_ref().is_ghost()
    }

    fn is_invulnerable(&self) -> bool {
        self.as_ref().is_invulnerable()
    }

    fn is_preset(&self) -> bool {
        self.as_ref().is_preset()
    }

    fn is_protected(&self) -> bool {
        self.as_ref().is_protected()
    }

    fn is_simple_actor(&self) -> bool {
        self.as_ref().is_simple_actor()
    }

    fn is_summonable(&self) -> bool {
        self.as_ref().is_summonable()
    }

    fn is_unique(&self) -> bool {
        self.as_ref().is_unique()
    }

    fn respawns(&self) -> bool {
        self.as_ref().respawns()
    }

    fn uses_opposite_gender_anims(&self) -> bool {
        self.as_ref().uses_opposite_gender_anims()
    }

    fn uses_template(&self) -> bool {
        self.as_ref().uses_template()
    }
}
