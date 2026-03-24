use bitflags::bitflags;
use core::ffi::c_void;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESFaction;
use crate::offsets::offsets_vtable::VTABLE_TESFaction;
use crate::re::AITimeStamp;
use crate::re::BGSListForm;
use crate::re::BGSOutfit;
use crate::re::BSFixedString;
use crate::re::BSHandleRefObject;
use crate::re::BSSimpleList;
use crate::re::BSTHashMap;
use crate::re::FIGHT_REACTION;
use crate::re::NiPointer;
use crate::re::NiRef;
use crate::re::PackageLocation;
use crate::re::ProcessLists;
use crate::re::TESCondition;
use crate::re::TESFullName;
use crate::re::TESNPC;
use crate::re::TESObjectREFR;
use crate::re::TESReactionForm;
use crate::re::TESTexture;
use crate::re::{FormCastable, FormType, TESForm};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};
use crate::version::RUNTIME_SSE_1_6_629;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FactionDataFlags: u32 {
        const HIDDEN_FROM_NPC = 1 << 0;
        const SPECIAL_COMBAT = 1 << 1;
        const PLAYER_IS_EXPELLED = 1 << 2;
        const PLAYER_IS_ENEMY = 1 << 3;
        const TRACK_CRIME = 1 << 6;
        const IGNORES_CRIMES_MURDER = 1 << 7;
        const IGNORES_CRIMES_ASSAULT = 1 << 8;
        const IGNORES_CRIMES_STEALING = 1 << 9;
        const IGNORES_CRIMES_TRESPASS = 1 << 10;
        const DO_NOT_REPORT_CRIMES_AGAINST_MEMBERS = 1 << 11;
        const CRIME_GOLD_USE_DEFAULTS = 1 << 12;
        const IGNORES_CRIMES_PICKPOCKET = 1 << 13;
        const VENDOR = 1 << 14;
        const CAN_BE_OWNER = 1 << 15;
        const IGNORES_CRIMES_WEREWOLF = 1 << 16;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESFactionChangeFlags: u32 {
        const FACTION_FLAGS = 1 << 1;
        const FACTION_REACTIONS = 1 << 2;
        const FACTION_CRIME_COUNTS = 1 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESFactionRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactionData {
    pub flags: FactionDataFlags, // 00
}

const _: () = assert!(core::mem::size_of::<FactionData>() == 0x4);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactionCrimeDataValues {
    pub arrest: bool,               // 00
    pub attack_on_sight: bool,      // 01
    pub murder_crime_gold: u16,     // 02
    pub assault_crime_gold: u16,    // 04
    pub trespass_crime_gold: u16,   // 06
    pub pickpocket_crime_gold: u16, // 08
    pub pad0a: u16,                 // 0A
    pub steal_crime_gold_mult: f32, // 0C
    pub escape_crime_gold: u16,     // 10
    pub werewolf_crime_gold: u16,   // 12
}

const _: () = assert!(core::mem::size_of::<FactionCrimeDataValues>() == 0x14);

#[repr(C)]
pub struct FactionCrimeData {
    pub faction_jail_marker: *mut TESObjectREFR,      // 00
    pub faction_wait_marker: *mut TESObjectREFR,      // 08
    pub faction_stolen_container: *mut TESObjectREFR, // 10
    pub faction_player_inventory_container: *mut TESObjectREFR, // 18
    pub crime_group: *mut BGSListForm,                // 20
    pub jail_outfit: *mut BGSOutfit,                  // 28
    pub crimevalues: FactionCrimeDataValues,          // 30
    pub pad44: u32,                                   // 44
}

const _: () = assert!(core::mem::size_of::<FactionCrimeData>() == 0x48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactionVendorDataValues {
    pub start_hour: u16,       // 00
    pub end_hour: u16,         // 02
    pub location_radius: u32,  // 04
    pub buys_stolen: bool,     // 08
    pub not_buy_sell: bool,    // 09
    pub buys_non_stolen: bool, // 0A
    pub pad0b: u8,             // 0B
}

const _: () = assert!(core::mem::size_of::<FactionVendorDataValues>() == 0x0C);

#[repr(C)]
pub struct FactionVendorData {
    pub vendor_values: FactionVendorDataValues, // 00
    pub pad0c: u32,                             // 0C
    pub vendor_location: *mut PackageLocation,  // 10
    pub vendor_conditions: *mut TESCondition,   // 18
    pub vendor_sell_buy_list: *mut BGSListForm, // 20
    pub merchant_container: *mut TESObjectREFR, // 28
    pub last_day_reset: u32,                    // 30
    pub pad34: u32,                             // 34
}

const _: () = assert!(core::mem::size_of::<FactionVendorData>() == 0x38);

#[repr(C)]
pub struct RankData {
    pub male_rank_title: BSFixedString,   // 00
    pub female_rank_title: BSFixedString, // 08
    pub texture_insignia: TESTexture,     // 10
}

const _: () = assert!(core::mem::size_of::<RankData>() == 0x20);

core_util::abstract_type! { type PlayerCharacter; }

impl NiRef for PlayerCharacter {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).inc_ref_count() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).dec_ref_count() };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CrimeGoldStruct {
    violent_cur: f32,        // 00
    non_violent_cur: f32,    // 04
    non_violent_infamy: f32, // 08
    violent_infamy: f32,     // 0C
}

const _: () = assert!(core::mem::size_of::<CrimeGoldStruct>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy)]
struct StolenItemValueStruct {
    unwitnessed: i32, // 00
    witnessed: i32,   // 04
}

const _: () = assert!(core::mem::size_of::<StolenItemValueStruct>() == 0x08);

#[repr(C)]
struct CrimeValue {
    crime_gold_map: BSTHashMap<*const TESFaction, CrimeGoldStruct>, // 00
    stolen_item_value_map: BSTHashMap<*const TESFaction, StolenItemValueStruct>, // 30
}

const _: () = assert!(core::mem::size_of::<CrimeValue>() == 0x60);

impl PlayerCharacter {
    crate::relocation_variable! {
        fn singleton() -> *mut NiPointer<PlayerCharacter> => RelocationID::new(517014, 403521), is_ptr
    }

    #[inline(always)]
    fn get_singleton() -> *mut PlayerCharacter {
        let singleton = Self::singleton();
        if singleton.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*singleton).get() }
        }
    }

    crate::runtime_data_accessor! {
        fn get_crime_value() -> CrimeValue {
            version: RUNTIME_SSE_1_6_629,
            se: 0x3E0,
            ae: 0x9D0,
            vr: 0x3E8
        }
    }

    crate::relocated_virtual_method! {
        const VFUNC_SET_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B5, 0x0B7);
        fn set_crime_gold_value(&mut self, faction: *mut TESFaction, violent: bool, amount: u32)
    }

    crate::relocated_virtual_method! {
        const VFUNC_MOD_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B6, 0x0B8);
        fn mod_crime_gold_value(&mut self, faction: *mut TESFaction, violent: bool, amount: i32)
    }

    crate::relocated_virtual_method! {
        const VFUNC_GET_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B8, 0x0BA);
        fn get_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    crate::relocated_virtual_method! {
        const VFUNC_GO_TO_PRISON: VariantOffset = VariantOffset::new_se_ae(0x0B9, 0x0BB);
        fn go_to_prison(&mut self, faction: *mut TESFaction, remove_inventory: bool, real_jail: bool)
    }

    crate::relocated_virtual_method! {
        const VFUNC_PAY_FINE: VariantOffset = VariantOffset::new_se_ae(0x0BB, 0x0BD);
        fn pay_fine(&mut self, faction: *mut TESFaction, go_to_jail: bool, remove_stolen_items: bool)
    }

    crate::relocated_virtual_method! {
        const VFUNC_GET_VIOLENT_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x12B, 0x12D);
        fn get_violent_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    crate::relocated_virtual_method! {
        const VFUNC_GET_NON_VIOLENT_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x12C, 0x12E);
        fn get_non_violent_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    #[inline(always)]
    fn get_gold_amount(&mut self, no_init: bool) -> i32 {
        unsafe {
            crate::ffi::commonlib_actor_get_gold_amount(self as *mut Self as *mut c_void, no_init)
        }
    }
}

#[repr(C)]
pub struct TESFaction {
    pub base: TESForm,                                       // 000
    pub full_name: TESFullName,                              // 020
    pub reaction_form: TESReactionForm,                      // 030
    pub crime_gold_map: *mut BSTHashMap<*const TESNPC, u32>, // 050
    pub data: FactionData,                                   // 058
    pub pad05c: u32,                                         // 05C
    pub crime_data: FactionCrimeData,                        // 060
    pub vendor_data: FactionVendorData,                      // 0A8
    pub rank_data: BSSimpleList<*mut RankData>,              // 0E0
    pub major_crime: i32,                                    // 0F0
    pub minor_crime: i32,                                    // 0F4
    pub resist_arrest_time_stamp: AITimeStamp,               // 0F8
    pub pc_enemy_flag_time_stamp: f32,                       // 0FC
}

const _: () = assert!(core::mem::size_of::<TESFaction>() == 0x100);
const _: () = assert!(core::mem::offset_of!(TESFaction, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESFaction, reaction_form) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESFaction, crime_gold_map) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESFaction, data) == 0x58);
const _: () = assert!(core::mem::offset_of!(TESFaction, crime_data) == 0x60);
const _: () = assert!(core::mem::offset_of!(TESFaction, vendor_data) == 0xA8);
const _: () = assert!(core::mem::offset_of!(TESFaction, rank_data) == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESFaction, resist_arrest_time_stamp) == 0xF8);

impl RttiType for TESFaction {
    const RTTI: VariantID = RTTI_TESFaction;
}

impl FormCastable for TESFaction {
    const TARGET_FORM_TYPE: FormType = FormType::Faction;
}

inherit!(TESFaction : TESForm);
inherit!(TESFaction => TESFullName, full_name);
inherit!(TESFaction => TESReactionForm, reaction_form);

impl TESFaction {
    pub const RTTI: VariantID = RTTI_TESFaction;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESFaction;
    pub const FORMTYPE: FormType = FormType::Faction;

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;    // 12
    // void InitItemImpl() override;                      // 13

    crate::relocation_func! {
        pub fn set_faction_fight_reaction(
            &mut self,
            faction: *mut TESFaction,
            fight_reaction: FIGHT_REACTION
        ) => RelocationID::new(24012, 24516)
    }

    #[inline]
    pub const fn can_be_owner(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::CAN_BE_OWNER)
    }

    pub fn can_pay_crime_gold(&self) -> bool {
        let player = PlayerCharacter::get_singleton();
        if player.is_null() {
            return false;
        }

        let player = unsafe { &mut *player };
        let bounty = player.get_crime_gold_value(self);
        player.get_gold_amount(false) >= bounty as i32
    }

    #[inline]
    fn crime_gold_struct(&self) -> Option<CrimeGoldStruct> {
        let player = unsafe { PlayerCharacter::get_singleton().as_ref() }?;
        let value = player
            .get_crime_value()
            .crime_gold_map
            .find(&(self as *const TESFaction));
        if value.is_null() {
            None
        } else {
            Some(unsafe { (*value).second })
        }
    }

    #[inline]
    fn stolen_item_value_struct(&self) -> Option<StolenItemValueStruct> {
        let player = unsafe { PlayerCharacter::get_singleton().as_ref() }?;
        let value = player
            .get_crime_value()
            .stolen_item_value_map
            .find(&(self as *const TESFaction));
        if value.is_null() {
            None
        } else {
            Some(unsafe { (*value).second })
        }
    }

    #[inline]
    pub fn get_crime_gold(&self) -> i32 {
        unsafe { PlayerCharacter::get_singleton().as_ref() }
            .map_or(0, |player| player.get_crime_gold_value(self) as i32)
    }

    #[inline]
    pub fn get_crime_gold_non_violent(&self) -> i32 {
        unsafe { PlayerCharacter::get_singleton().as_ref() }.map_or(0, |player| {
            player.get_non_violent_crime_gold_value(self) as i32
        })
    }

    #[inline]
    pub fn get_crime_gold_violent(&self) -> i32 {
        unsafe { PlayerCharacter::get_singleton().as_ref() }
            .map_or(0, |player| player.get_violent_crime_gold_value(self) as i32)
    }

    #[inline]
    pub fn get_infamy(&self) -> i32 {
        self.crime_gold_struct().map_or(0, |value| {
            (value.non_violent_infamy + value.violent_infamy) as i32
        })
    }

    #[inline]
    pub fn get_infamy_non_violent(&self) -> i32 {
        self.crime_gold_struct()
            .map_or(0, |value| value.non_violent_infamy as i32)
    }

    #[inline]
    pub fn get_infamy_violent(&self) -> i32 {
        self.crime_gold_struct()
            .map_or(0, |value| value.violent_infamy as i32)
    }

    #[inline]
    pub fn get_stolen_item_value_crime(&self) -> i32 {
        self.stolen_item_value_struct()
            .map_or(0, |value| value.witnessed)
    }

    #[inline]
    pub fn get_stolen_item_value_no_crime(&self) -> i32 {
        self.stolen_item_value_struct()
            .map_or(0, |value| value.unwitnessed)
    }

    #[inline]
    pub const fn has_special_combat_state(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::SPECIAL_COMBAT)
    }

    #[inline]
    pub const fn has_steal_multiplier(&self) -> bool {
        self.crime_data.crimevalues.steal_crime_gold_mult > 0.0
    }

    #[inline]
    pub const fn hidden_from_npc(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::HIDDEN_FROM_NPC)
    }

    #[inline]
    pub const fn ignores_assault(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_ASSAULT)
    }

    #[inline]
    pub const fn ignores_murder(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_MURDER)
    }

    #[inline]
    pub const fn ignores_pickpocket(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_PICKPOCKET)
    }

    #[inline]
    pub const fn ignores_stealing(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_STEALING)
    }

    #[inline]
    pub const fn ignores_trespass(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_TRESPASS)
    }

    #[inline]
    pub const fn ignores_werewolf(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::IGNORES_CRIMES_WEREWOLF)
    }

    #[inline]
    pub fn is_faction_in_crime_group(&self, other: *const TESFaction) -> bool {
        let Some(list) = (unsafe { self.crime_data.crime_group.as_ref() }) else {
            return false;
        };
        if other.is_null() {
            return false;
        }
        list.has_form(other.cast_mut().cast())
    }

    #[inline]
    pub const fn is_player_enemy(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::PLAYER_IS_ENEMY)
    }

    #[inline]
    pub const fn is_player_expelled(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::PLAYER_IS_EXPELLED)
    }

    #[inline]
    pub const fn is_vendor(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::VENDOR)
    }

    #[inline]
    pub const fn offers_services(&self) -> bool {
        !self.vendor_data.vendor_sell_buy_list.is_null()
    }

    #[inline]
    pub const fn reports_crimes_against_members(&self) -> bool {
        !self
            .data
            .flags
            .contains(FactionDataFlags::DO_NOT_REPORT_CRIMES_AGAINST_MEMBERS)
    }

    #[inline]
    pub const fn tracks_crimes(&self) -> bool {
        self.data.flags.contains(FactionDataFlags::TRACK_CRIME)
    }

    #[inline]
    pub const fn uses_crime_gold_defaults(&self) -> bool {
        self.data
            .flags
            .contains(FactionDataFlags::CRIME_GOLD_USE_DEFAULTS)
    }

    pub fn set_ally(
        &mut self,
        other: *mut TESFaction,
        self_is_friend_to_other: bool,
        other_is_friend_to_self: bool,
    ) {
        if other.is_null() {
            return;
        }

        let self_reaction = if self_is_friend_to_other {
            FIGHT_REACTION::Friend
        } else {
            FIGHT_REACTION::Ally
        };
        let other_reaction = if other_is_friend_to_self {
            FIGHT_REACTION::Friend
        } else {
            FIGHT_REACTION::Ally
        };

        self.set_faction_fight_reaction(other, self_reaction);
        unsafe { (*other).set_faction_fight_reaction(self, other_reaction) };

        let process_lists = ProcessLists::get_singleton();
        if !process_lists.is_null() {
            unsafe { (*process_lists).clear_cached_faction_fight_reactions() };
        }
    }

    #[inline]
    pub fn mod_crime_gold(&mut self, amount: i32, violent: bool) {
        let player = PlayerCharacter::get_singleton();
        if !player.is_null() {
            unsafe { (*player).mod_crime_gold_value(self, violent, amount) };
        }
    }

    #[inline]
    pub fn player_pay_crime_gold(&mut self, remove_stolen_items: bool, go_to_jail: bool) {
        let player = PlayerCharacter::get_singleton();
        if !player.is_null() {
            unsafe { (*player).pay_fine(self, go_to_jail, remove_stolen_items) };
        }
    }

    #[inline]
    pub fn send_player_to_jail(&mut self, remove_inventory: bool, real_jail: bool) {
        let player = PlayerCharacter::get_singleton();
        if !player.is_null() {
            unsafe { (*player).go_to_prison(self, remove_inventory, real_jail) };
        }
    }

    pub fn set_enemy(
        &mut self,
        other: *mut TESFaction,
        self_is_neutral_to_other: bool,
        other_is_neutral_to_self: bool,
    ) {
        if other.is_null() {
            return;
        }

        let self_reaction = if self_is_neutral_to_other {
            FIGHT_REACTION::Neutral
        } else {
            FIGHT_REACTION::Enemy
        };
        let other_reaction = if other_is_neutral_to_self {
            FIGHT_REACTION::Neutral
        } else {
            FIGHT_REACTION::Enemy
        };

        self.set_faction_fight_reaction(other, self_reaction);
        unsafe { (*other).set_faction_fight_reaction(self, other_reaction) };

        let process_lists = ProcessLists::get_singleton();
        if !process_lists.is_null() {
            unsafe { (*process_lists).clear_cached_faction_fight_reactions() };
        }
    }

    #[inline]
    pub fn set_crime_gold(&mut self, gold: i32) {
        let player = PlayerCharacter::get_singleton();
        if !player.is_null() {
            unsafe { (*player).set_crime_gold_value(self, false, gold as u32) };
        }
    }

    #[inline]
    pub fn set_crime_gold_violent(&mut self, gold: i32) {
        let player = PlayerCharacter::get_singleton();
        if !player.is_null() {
            unsafe { (*player).set_crime_gold_value(self, true, gold as u32) };
        }
    }
}
