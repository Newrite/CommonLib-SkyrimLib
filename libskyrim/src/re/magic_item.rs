use crate::offsets::offsets_rtti::{RTTI_MagicItem, RTTI_MagicItem__PreloadableVisitor};
use crate::offsets::offsets_vtable::{VTABLE_MagicItem, VTABLE_MagicItem__PreloadableVisitor};
use crate::re::Actor;
use crate::re::Effect;
use crate::re::EffectArchetypeId;
use crate::re::EffectItem;
use crate::re::EffectSetting;
use crate::re::MagicItemDataCollector;
use crate::re::MagicItemTraversalFunctor;
use crate::re::QueuedFile;
use crate::re::TESBoundObject;
use crate::re::TESFile;
use crate::re::TESFullName;
use crate::re::TESModel;
use crate::re::TESObjectWEAP;
use crate::re::actor_values::ActorValue;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bst_array::BSTArray;
use crate::re::magic_system::{CastingType, Delivery, SpellType};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

/// C++ `RE::MagicItem::PreloadableVisitor`
#[repr(C)]
pub struct MagicItemPreloadableVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<MagicItemPreloadableVisitor>() == 0x8);

impl RttiType for MagicItemPreloadableVisitor {
    const RTTI: VariantID = RTTI_MagicItem__PreloadableVisitor;
}

impl MagicItemPreloadableVisitor {
    pub const RTTI: VariantID = RTTI_MagicItem__PreloadableVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicItem__PreloadableVisitor;

    virtual_method! {
        pub const VFUNC_VISIT_MODEL: usize = 0x00;
        pub fn visit_model(model: *mut TESModel)
    }

    virtual_method! {
        pub const VFUNC_VISIT_WEAPON: usize = 0x01;
        pub fn visit_weapon(weapon: *mut TESObjectWEAP)
    }
}

/// C++ `RE::MagicItem::SkillUsageData`
#[repr(C)]
pub struct MagicItemSkillUsageData {
    pub effect: *mut EffectItem, // 00
    pub skill: ActorValue,       // 08
    pub magnitude: f32,          // 0C
    pub custom: bool,            // 10
    pub pad11: u8,               // 11
    pub pad12: u16,              // 12
    pub pad14: u32,              // 14
}

const _: () = assert!(core::mem::size_of::<MagicItemSkillUsageData>() == 0x18);

/// C++ `RE::MagicItem::Data`
#[repr(C)]
pub struct MagicItemData {
    pub cost_override: i32, // 00
    pub flags: u32,         // 04
}

const _: () = assert!(core::mem::size_of::<MagicItemData>() == 0x08);

/// C++ `RE::MagicItem`
#[repr(C)]
pub struct MagicItem {
    pub base: TESBoundObject,                  // 00
    pub full_name: TESFullName,                // 30
    pub keyword_form: BGSKeywordForm,          // 40
    pub effects: BSTArray<*mut Effect>,        // 58
    pub hostile_count: i32,                    // 70
    pub pad74: u32,                            // 74
    pub av_effect_setting: *mut EffectSetting, // 78
    pub preload_count: u32,                    // 80
    pub pad84: u32,                            // 84
    pub preloaded_item: *mut QueuedFile,       // 88 - BSTSmartPointer<QueuedFile>
}

const _: () = assert!(core::mem::size_of::<MagicItem>() == 0x90);
const _: () = assert!(core::mem::offset_of!(MagicItem, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(MagicItem, keyword_form) == 0x40);
const _: () = assert!(core::mem::offset_of!(MagicItem, effects) == 0x58);
const _: () = assert!(core::mem::offset_of!(MagicItem, hostile_count) == 0x70);
const _: () = assert!(core::mem::offset_of!(MagicItem, av_effect_setting) == 0x78);
const _: () = assert!(core::mem::offset_of!(MagicItem, preloaded_item) == 0x88);

inherit!(MagicItem : TESBoundObject);
inherit!(MagicItem => TESFullName, full_name);
inherit!(MagicItem => BGSKeywordForm, keyword_form);

impl RttiType for MagicItem {
    const RTTI: VariantID = RTTI_MagicItem;
}

impl AsRef<MagicItem> for MagicItem {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MagicItem> for MagicItem {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl MagicItem {
    pub const RTTI: VariantID = RTTI_MagicItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicItem;

    virtual_method! {
        pub const VFUNC_GET_SPELL_TYPE: usize = 0x53;
        pub fn get_spell_type() -> SpellType
    }

    virtual_method! {
        pub const VFUNC_SET_CASTING_TYPE: usize = 0x54;
        pub fn set_casting_type(casting_type: CastingType)
    }

    virtual_method! {
        pub const VFUNC_GET_CASTING_TYPE: usize = 0x55;
        pub fn get_casting_type() -> CastingType
    }

    virtual_method! {
        pub const VFUNC_SET_DELIVERY: usize = 0x56;
        pub fn set_delivery(delivery: Delivery)
    }

    virtual_method! {
        pub const VFUNC_GET_DELIVERY: usize = 0x57;
        pub fn get_delivery() -> Delivery
    }

    virtual_method! {
        pub const VFUNC_IS_VALID_DELIVERY: usize = 0x58;
        pub fn is_valid_delivery(delivery: Delivery) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_FIXED_CAST_DURATION: usize = 0x59;
        pub fn get_fixed_cast_duration() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_RANGE: usize = 0x5A;
        pub fn get_range() -> f32
    }

    virtual_method! {
        pub const VFUNC_IGNORES_RESISTANCE: usize = 0x5B;
        pub fn ignores_resistance() -> bool
    }

    virtual_method! {
        pub const VFUNC_IGNORE_LOS: usize = 0x5C;
        pub fn ignore_los() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_FOOD: usize = 0x5D;
        pub fn is_food() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_NO_ABSORB: usize = 0x5E;
        pub fn get_no_absorb() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_NO_DUAL_CAST_MODIFICATIONS: usize = 0x5F;
        pub fn get_no_dual_cast_modifications() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_SKILL_USAGE_DATA: usize = 0x60;
        pub fn get_skill_usage_data(data: *mut MagicItemSkillUsageData) -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_POISON: usize = 0x61;
        pub fn is_poison() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_MEDICINE: usize = 0x62;
        pub fn is_medicine() -> bool
    }

    virtual_method! {
        pub const VFUNC_ADJUST_COST: usize = 0x63;
        pub fn adjust_cost(cost: *mut f32, actor: *mut Actor)
    }

    virtual_method! {
        pub const VFUNC_GET_CHARGE_TIME: usize = 0x64;
        pub fn get_charge_time() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_MAX_EFFECT_COUNT: usize = 0x65;
        pub fn get_max_effect_count() -> u32
    }

    virtual_method! {
        pub const VFUNC_GET_ASSOCIATED_SKILL: usize = 0x66;
        pub fn get_associated_skill() -> ActorValue
    }

    virtual_method! {
        pub const VFUNC_IS_TWO_HANDED: usize = 0x67;
        pub fn is_two_handed() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_CHUNK_ID: usize = 0x68;
        pub fn get_chunk_id() -> u32
    }

    virtual_method! {
        pub const VFUNC_COPY_MAGIC_ITEM_DATA: usize = 0x69;
        pub fn copy_magic_item_data(src: *mut MagicItem)
    }

    virtual_method! {
        pub const VFUNC_LOAD_MAGIC_ITEM_CHUNK: usize = 0x6A;
        pub fn load_magic_item_chunk(mod_file: *mut TESFile, chunk_id: u32)
    }

    virtual_method! {
        pub const VFUNC_LOAD_CHUNK_DATA_POST_PROCESS: usize = 0x6B;
        pub fn load_chunk_data_post_process(mod_file: *mut TESFile)
    }

    virtual_method! {
        pub const VFUNC_GET_DATA1: usize = 0x6C;
        pub fn get_data1() -> *const MagicItemData
    }

    virtual_method! {
        pub const VFUNC_GET_DATA2: usize = 0x6D;
        pub fn get_data2() -> *mut MagicItemData
    }

    virtual_method! {
        pub const VFUNC_GET_DATA_SIZE: usize = 0x6E;
        pub fn get_data_size() -> u32
    }

    virtual_method! {
        pub const VFUNC_INIT_FROM_CHUNK: usize = 0x6F;
        pub fn init_from_chunk(mod_file: *mut TESFile)
    }

    virtual_method! {
        pub const VFUNC_INIT_CHUNK: usize = 0x70;
        pub fn init_chunk()
    }

    #[inline(always)]
    pub fn calculate_magicka_cost(&self, caster: *mut Actor) -> f32 {
        self.calculate_cost(caster)
    }

    #[inline(always)]
    pub fn calculate_total_gold_value(&self, caster: *mut Actor) -> f32 {
        self.calculate_cost(caster)
    }

    #[inline]
    pub fn collect_data(&self) -> MagicItemDataCollector {
        let mut result = MagicItemDataCollector::new(self);
        self.traverse(&mut result);
        result
    }

    crate::relocation_func! {
        pub fn get_av_effect(&self) -> *mut EffectSetting => RelocationID::new(11194, 11302)
    }

    crate::relocation_func! {
        pub fn is_valid(&self) -> bool => RelocationID::new(11183, 11290)
    }

    crate::relocation_func! {
        pub fn get_costliest_effect_item(&self, delivery: Delivery, positive_area: bool) -> *mut Effect => RelocationID::new(11216, 11335)
    }

    crate::relocation_func! {
        fn calculate_cost(&self, caster: *mut Actor) -> f32 => RelocationID::new(11213, 11321)
    }

    #[inline(always)]
    pub fn get_data(&self) -> *const MagicItemData {
        self.get_data1()
    }

    #[inline(always)]
    pub fn get_data_mut(&mut self) -> *mut MagicItemData {
        self.get_data2()
    }

    crate::relocation_func! {
        pub fn get_largest_area(&self) -> i32 => RelocationID::new(11219, 11338)
    }

    crate::relocation_func! {
        pub fn get_longest_duration(&self) -> u32 => RelocationID::new(11218, 11337)
    }

    crate::relocation_func! {
        pub fn has_effect(&mut self, archetype: EffectArchetypeId) -> bool => RelocationID::new(11207, 11315)
    }

    #[inline(always)]
    pub fn is_hostile(&self) -> bool {
        self.hostile_count > 0
    }

    crate::relocation_func! {
        pub fn is_permanent(&self) -> bool => RelocationID::new(11183, 11290)
    }

    pub fn get_effect_is_match(
        &self,
        base: *mut EffectSetting,
        mag: f32,
        area: u32,
        dur: u32,
        cost: f32,
    ) -> *mut Effect {
        for &effect in unsafe { self.effects.as_slice() } {
            if unsafe { effect.as_ref() }
                .map(|effect| effect.is_match(base, mag, area, dur, cost))
                .unwrap_or(false)
            {
                return effect;
            }
        }

        core::ptr::null_mut()
    }

    crate::relocation_func! {
        pub fn traverse(&self, visitor: &mut MagicItemTraversalFunctor) => RelocationID::new(11222, 11341)
    }
}

pub trait MagicItemExt {
    fn get_spell_type(&self) -> SpellType;
    fn set_casting_type(&mut self, casting_type: CastingType);
    fn get_casting_type(&self) -> CastingType;
    fn set_delivery(&mut self, delivery: Delivery);
    fn get_delivery(&self) -> Delivery;
    fn is_valid_delivery(&self, delivery: Delivery) -> bool;
    fn get_fixed_cast_duration(&self) -> f32;
    fn get_range(&self) -> f32;
    fn ignores_resistance(&self) -> bool;
    fn ignore_los(&self) -> bool;
    fn is_food(&self) -> bool;
    fn get_no_absorb(&self) -> bool;
    fn get_no_dual_cast_modifications(&self) -> bool;
    fn get_skill_usage_data(&self, data: *mut MagicItemSkillUsageData) -> bool;
    fn is_poison(&self) -> bool;
    fn is_medicine(&self) -> bool;
    fn adjust_cost(&self, cost: *mut f32, actor: *mut Actor);
    fn get_charge_time(&self) -> f32;
    fn get_max_effect_count(&self) -> u32;
    fn get_associated_skill(&self) -> ActorValue;
    fn is_two_handed(&self) -> bool;
    fn get_chunk_id(&mut self) -> u32;
    fn copy_magic_item_data(&mut self, src: *mut MagicItem);
    fn load_magic_item_chunk(&mut self, mod_file: *mut TESFile, chunk_id: u32);
    fn load_chunk_data_post_process(&mut self, mod_file: *mut TESFile);
    fn get_data1(&self) -> *const MagicItemData;
    fn get_data2(&mut self) -> *mut MagicItemData;
    fn get_data_size(&self) -> u32;
    fn init_from_chunk(&mut self, mod_file: *mut TESFile);
    fn init_chunk(&mut self);
    fn calculate_magicka_cost(&self, caster: *mut Actor) -> f32;
    fn calculate_total_gold_value(&self, caster: *mut Actor) -> f32;
    fn collect_data(&self) -> MagicItemDataCollector;
    fn get_av_effect(&self) -> *mut EffectSetting;
    fn is_valid(&self) -> bool;
    fn get_costliest_effect_item(&self, delivery: Delivery, positive_area: bool) -> *mut Effect;
    fn get_data(&self) -> *const MagicItemData;
    fn get_data_mut(&mut self) -> *mut MagicItemData;
    fn get_largest_area(&self) -> i32;
    fn get_longest_duration(&self) -> u32;
    fn has_effect(&mut self, archetype: EffectArchetypeId) -> bool;
    fn is_hostile(&self) -> bool;
    fn is_permanent(&self) -> bool;
    fn get_effect_is_match(
        &self,
        base: *mut EffectSetting,
        mag: f32,
        area: u32,
        dur: u32,
        cost: f32,
    ) -> *mut Effect;
    fn traverse(&self, visitor: &mut MagicItemTraversalFunctor);
}

impl<T: AsRef<MagicItem> + AsMut<MagicItem>> MagicItemExt for T {
    fn get_spell_type(&self) -> SpellType {
        self.as_ref().get_spell_type()
    }

    fn set_casting_type(&mut self, casting_type: CastingType) {
        MagicItem::set_casting_type(self.as_mut(), casting_type)
    }

    fn get_casting_type(&self) -> CastingType {
        self.as_ref().get_casting_type()
    }

    fn set_delivery(&mut self, delivery: Delivery) {
        MagicItem::set_delivery(self.as_mut(), delivery)
    }

    fn get_delivery(&self) -> Delivery {
        self.as_ref().get_delivery()
    }

    fn is_valid_delivery(&self, delivery: Delivery) -> bool {
        self.as_ref().is_valid_delivery(delivery)
    }

    fn get_fixed_cast_duration(&self) -> f32 {
        self.as_ref().get_fixed_cast_duration()
    }

    fn get_range(&self) -> f32 {
        self.as_ref().get_range()
    }

    fn ignores_resistance(&self) -> bool {
        self.as_ref().ignores_resistance()
    }

    fn ignore_los(&self) -> bool {
        self.as_ref().ignore_los()
    }

    fn is_food(&self) -> bool {
        self.as_ref().is_food()
    }

    fn get_no_absorb(&self) -> bool {
        self.as_ref().get_no_absorb()
    }

    fn get_no_dual_cast_modifications(&self) -> bool {
        self.as_ref().get_no_dual_cast_modifications()
    }

    fn get_skill_usage_data(&self, data: *mut MagicItemSkillUsageData) -> bool {
        self.as_ref().get_skill_usage_data(data)
    }

    fn is_poison(&self) -> bool {
        self.as_ref().is_poison()
    }

    fn is_medicine(&self) -> bool {
        self.as_ref().is_medicine()
    }

    fn adjust_cost(&self, cost: *mut f32, actor: *mut Actor) {
        self.as_ref().adjust_cost(cost, actor)
    }

    fn get_charge_time(&self) -> f32 {
        self.as_ref().get_charge_time()
    }

    fn get_max_effect_count(&self) -> u32 {
        self.as_ref().get_max_effect_count()
    }

    fn get_associated_skill(&self) -> ActorValue {
        self.as_ref().get_associated_skill()
    }

    fn is_two_handed(&self) -> bool {
        self.as_ref().is_two_handed()
    }

    fn get_chunk_id(&mut self) -> u32 {
        MagicItem::get_chunk_id(self.as_mut())
    }

    fn copy_magic_item_data(&mut self, src: *mut MagicItem) {
        MagicItem::copy_magic_item_data(self.as_mut(), src)
    }

    fn load_magic_item_chunk(&mut self, mod_file: *mut TESFile, chunk_id: u32) {
        MagicItem::load_magic_item_chunk(self.as_mut(), mod_file, chunk_id)
    }

    fn load_chunk_data_post_process(&mut self, mod_file: *mut TESFile) {
        MagicItem::load_chunk_data_post_process(self.as_mut(), mod_file)
    }

    fn get_data1(&self) -> *const MagicItemData {
        self.as_ref().get_data1()
    }

    fn get_data2(&mut self) -> *mut MagicItemData {
        MagicItem::get_data2(self.as_mut())
    }

    fn get_data_size(&self) -> u32 {
        self.as_ref().get_data_size()
    }

    fn init_from_chunk(&mut self, mod_file: *mut TESFile) {
        MagicItem::init_from_chunk(self.as_mut(), mod_file)
    }

    fn init_chunk(&mut self) {
        MagicItem::init_chunk(self.as_mut())
    }

    fn calculate_magicka_cost(&self, caster: *mut Actor) -> f32 {
        self.as_ref().calculate_magicka_cost(caster)
    }

    fn calculate_total_gold_value(&self, caster: *mut Actor) -> f32 {
        self.as_ref().calculate_total_gold_value(caster)
    }

    fn collect_data(&self) -> MagicItemDataCollector {
        self.as_ref().collect_data()
    }

    fn get_av_effect(&self) -> *mut EffectSetting {
        self.as_ref().get_av_effect()
    }

    fn is_valid(&self) -> bool {
        self.as_ref().is_valid()
    }

    fn get_costliest_effect_item(&self, delivery: Delivery, positive_area: bool) -> *mut Effect {
        self.as_ref()
            .get_costliest_effect_item(delivery, positive_area)
    }

    fn get_data(&self) -> *const MagicItemData {
        self.as_ref().get_data()
    }

    fn get_data_mut(&mut self) -> *mut MagicItemData {
        self.as_mut().get_data_mut()
    }

    fn get_largest_area(&self) -> i32 {
        self.as_ref().get_largest_area()
    }

    fn get_longest_duration(&self) -> u32 {
        self.as_ref().get_longest_duration()
    }

    fn has_effect(&mut self, archetype: EffectArchetypeId) -> bool {
        MagicItem::has_effect(self.as_mut(), archetype)
    }

    fn is_hostile(&self) -> bool {
        self.as_ref().is_hostile()
    }

    fn is_permanent(&self) -> bool {
        self.as_ref().is_permanent()
    }

    fn get_effect_is_match(
        &self,
        base: *mut EffectSetting,
        mag: f32,
        area: u32,
        dur: u32,
        cost: f32,
    ) -> *mut Effect {
        self.as_ref()
            .get_effect_is_match(base, mag, area, dur, cost)
    }

    fn traverse(&self, visitor: &mut MagicItemTraversalFunctor) {
        self.as_ref().traverse(visitor)
    }
}
