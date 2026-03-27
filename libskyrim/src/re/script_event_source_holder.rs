use crate::re::bgs_event_processed_event::BGSEventProcessedEvent;
use crate::re::bs_core_types::FormID;
use crate::re::{
    BSTEventSink, BSTEventSource, NiPointer, TESActivateEvent, TESActiveEffectApplyRemoveEvent,
    TESActorLocationChangeEvent, TESBookReadEvent, TESCellAttachDetachEvent,
    TESCellFullyLoadedEvent, TESCellReadyToApplyDecalsEvent, TESCombatEvent,
    TESContainerChangedEvent, TESDeathEvent, TESDestructionStageChangedEvent,
    TESEnterBleedoutEvent, TESEquipEvent, TESFastTravelEndEvent, TESFormDeleteEvent,
    TESFurnitureEvent, TESGrabReleaseEvent, TESHitEvent, TESInitScriptEvent, TESLoadGameEvent,
    TESLockChangedEvent, TESMagicEffectApplyEvent, TESMagicWardHitEvent, TESMoveAttachDetachEvent,
    TESObjectLoadedEvent, TESObjectREFR, TESObjectREFRTranslationEvent, TESOpenCloseEvent,
    TESPackageEvent, TESPerkEntryRunEvent, TESPlayerBowShotEvent, TESQuestInitEvent,
    TESQuestStageEvent, TESQuestStageItemDoneEvent, TESQuestStartStopEvent, TESResetEvent,
    TESResolveNPCTemplatesEvent, TESSceneActionEvent, TESSceneEvent, TESScenePhaseEvent,
    TESSellEvent, TESSleepStartEvent, TESSleepStopEvent, TESSpellCastEvent,
    TESSwitchRaceCompleteEvent, TESTopicInfoEvent, TESTrackedStatsEvent, TESTrapHitEvent,
    TESTriggerEnterEvent, TESTriggerEvent, TESTriggerLeaveEvent, TESUniqueIDChangeEvent,
    TESWaitStartEvent, TESWaitStopEvent,
};

/// C++ `RE::ScriptEventSourceHolder`
///
/// The main `repr(C)` type keeps the common fixed base surface present across
/// SE/AE/VR. `TESFastTravelEndEvent` is a flat-only trailing base at `0x1238`
/// and is exposed through a runtime-aware accessor instead of a fake universal
/// field.
#[repr(C)]
pub struct ScriptEventSourceHolder {
    pub bgs_event_processed_event_source: BSTEventSource<BGSEventProcessedEvent>, // 0000
    pub tes_activate_event_source: BSTEventSource<TESActivateEvent>,              // 0058
    pub tes_active_effect_apply_remove_event_source:
        BSTEventSource<TESActiveEffectApplyRemoveEvent>, // 00B0
    pub tes_actor_location_change_event_source: BSTEventSource<TESActorLocationChangeEvent>, // 0108
    pub tes_book_read_event_source: BSTEventSource<TESBookReadEvent>,             // 0160
    pub tes_cell_attach_detach_event_source: BSTEventSource<TESCellAttachDetachEvent>, // 01B8
    pub tes_cell_fully_loaded_event_source: BSTEventSource<TESCellFullyLoadedEvent>, // 0210
    pub tes_cell_ready_to_apply_decals_event_source: BSTEventSource<TESCellReadyToApplyDecalsEvent>, // 0268
    pub tes_combat_event_source: BSTEventSource<TESCombatEvent>, // 02C0
    pub tes_container_changed_event_source: BSTEventSource<TESContainerChangedEvent>, // 0318
    pub tes_death_event_source: BSTEventSource<TESDeathEvent>,   // 0370
    pub tes_destruction_stage_changed_event_source: BSTEventSource<TESDestructionStageChangedEvent>, // 03C8
    pub tes_enter_bleedout_event_source: BSTEventSource<TESEnterBleedoutEvent>, // 0420
    pub tes_equip_event_source: BSTEventSource<TESEquipEvent>,                  // 0478
    pub tes_form_delete_event_source: BSTEventSource<TESFormDeleteEvent>,       // 04D0
    pub tes_furniture_event_source: BSTEventSource<TESFurnitureEvent>,          // 0528
    pub tes_grab_release_event_source: BSTEventSource<TESGrabReleaseEvent>,     // 0580
    pub tes_hit_event_source: BSTEventSource<TESHitEvent>,                      // 05D8
    pub tes_init_script_event_source: BSTEventSource<TESInitScriptEvent>,       // 0630
    pub tes_load_game_event_source: BSTEventSource<TESLoadGameEvent>,           // 0688
    pub tes_lock_changed_event_source: BSTEventSource<TESLockChangedEvent>,     // 06E0
    pub tes_magic_effect_apply_event_source: BSTEventSource<TESMagicEffectApplyEvent>, // 0738
    pub tes_magic_ward_hit_event_source: BSTEventSource<TESMagicWardHitEvent>,  // 0790
    pub tes_move_attach_detach_event_source: BSTEventSource<TESMoveAttachDetachEvent>, // 07E8
    pub tes_object_loaded_event_source: BSTEventSource<TESObjectLoadedEvent>,   // 0840
    pub tes_object_refr_translation_event_source: BSTEventSource<TESObjectREFRTranslationEvent>, // 0898
    pub tes_open_close_event_source: BSTEventSource<TESOpenCloseEvent>, // 08F0
    pub tes_package_event_source: BSTEventSource<TESPackageEvent>,      // 0948
    pub tes_perk_entry_run_event_source: BSTEventSource<TESPerkEntryRunEvent>, // 09A0
    pub tes_quest_init_event_source: BSTEventSource<TESQuestInitEvent>, // 09F8
    pub tes_quest_stage_event_source: BSTEventSource<TESQuestStageEvent>, // 0A50
    pub tes_quest_stage_item_done_event_source: BSTEventSource<TESQuestStageItemDoneEvent>, // 0AA8
    pub tes_quest_start_stop_event_source: BSTEventSource<TESQuestStartStopEvent>, // 0B00
    pub tes_reset_event_source: BSTEventSource<TESResetEvent>,          // 0B58
    pub tes_resolve_npc_templates_event_source: BSTEventSource<TESResolveNPCTemplatesEvent>, // 0BB0
    pub tes_scene_event_source: BSTEventSource<TESSceneEvent>,          // 0C08
    pub tes_scene_action_event_source: BSTEventSource<TESSceneActionEvent>, // 0C60
    pub tes_scene_phase_event_source: BSTEventSource<TESScenePhaseEvent>, // 0CB8
    pub tes_sell_event_source: BSTEventSource<TESSellEvent>,            // 0D10
    pub tes_sleep_start_event_source: BSTEventSource<TESSleepStartEvent>, // 0D68
    pub tes_sleep_stop_event_source: BSTEventSource<TESSleepStopEvent>, // 0DC0
    pub tes_spell_cast_event_source: BSTEventSource<TESSpellCastEvent>, // 0E18
    pub tes_player_bow_shot_event_source: BSTEventSource<TESPlayerBowShotEvent>, // 0E70
    pub tes_topic_info_event_source: BSTEventSource<TESTopicInfoEvent>, // 0EC8
    pub tes_tracked_stats_event_source: BSTEventSource<TESTrackedStatsEvent>, // 0F20
    pub tes_trap_hit_event_source: BSTEventSource<TESTrapHitEvent>,     // 0F78
    pub tes_trigger_event_source: BSTEventSource<TESTriggerEvent>,      // 0FD0
    pub tes_trigger_enter_event_source: BSTEventSource<TESTriggerEnterEvent>, // 1028
    pub tes_trigger_leave_event_source: BSTEventSource<TESTriggerLeaveEvent>, // 1080
    pub tes_unique_id_change_event_source: BSTEventSource<TESUniqueIDChangeEvent>, // 10D8
    pub tes_wait_start_event_source: BSTEventSource<TESWaitStartEvent>, // 1130
    pub tes_wait_stop_event_source: BSTEventSource<TESWaitStopEvent>,   // 1188
    pub tes_switch_race_complete_event_source: BSTEventSource<TESSwitchRaceCompleteEvent>, // 11E0
}

const _: () = assert!(core::mem::size_of::<ScriptEventSourceHolder>() == 0x1238);

pub trait ScriptEventSourceHolderEvent: Sized {
    fn event_source(holder: &ScriptEventSourceHolder) -> *mut BSTEventSource<Self>;
}

macro_rules! impl_script_event_source_holder_event {
    ($ty:ty, $field:ident) => {
        impl ScriptEventSourceHolderEvent for $ty {
            #[inline(always)]
            fn event_source(holder: &ScriptEventSourceHolder) -> *mut BSTEventSource<Self> {
                core::ptr::from_ref(&holder.$field).cast_mut()
            }
        }
    };
}

impl_script_event_source_holder_event!(BGSEventProcessedEvent, bgs_event_processed_event_source);
impl_script_event_source_holder_event!(TESActivateEvent, tes_activate_event_source);
impl_script_event_source_holder_event!(
    TESActiveEffectApplyRemoveEvent,
    tes_active_effect_apply_remove_event_source
);
impl_script_event_source_holder_event!(
    TESActorLocationChangeEvent,
    tes_actor_location_change_event_source
);
impl_script_event_source_holder_event!(TESBookReadEvent, tes_book_read_event_source);
impl_script_event_source_holder_event!(
    TESCellAttachDetachEvent,
    tes_cell_attach_detach_event_source
);
impl_script_event_source_holder_event!(TESCellFullyLoadedEvent, tes_cell_fully_loaded_event_source);
impl_script_event_source_holder_event!(
    TESCellReadyToApplyDecalsEvent,
    tes_cell_ready_to_apply_decals_event_source
);
impl_script_event_source_holder_event!(TESCombatEvent, tes_combat_event_source);
impl_script_event_source_holder_event!(
    TESContainerChangedEvent,
    tes_container_changed_event_source
);
impl_script_event_source_holder_event!(TESDeathEvent, tes_death_event_source);
impl_script_event_source_holder_event!(
    TESDestructionStageChangedEvent,
    tes_destruction_stage_changed_event_source
);
impl_script_event_source_holder_event!(TESEnterBleedoutEvent, tes_enter_bleedout_event_source);
impl_script_event_source_holder_event!(TESEquipEvent, tes_equip_event_source);
impl_script_event_source_holder_event!(TESFormDeleteEvent, tes_form_delete_event_source);
impl_script_event_source_holder_event!(TESFurnitureEvent, tes_furniture_event_source);
impl_script_event_source_holder_event!(TESGrabReleaseEvent, tes_grab_release_event_source);
impl_script_event_source_holder_event!(TESHitEvent, tes_hit_event_source);
impl_script_event_source_holder_event!(TESInitScriptEvent, tes_init_script_event_source);
impl_script_event_source_holder_event!(TESLoadGameEvent, tes_load_game_event_source);
impl_script_event_source_holder_event!(TESLockChangedEvent, tes_lock_changed_event_source);
impl_script_event_source_holder_event!(
    TESMagicEffectApplyEvent,
    tes_magic_effect_apply_event_source
);
impl_script_event_source_holder_event!(TESMagicWardHitEvent, tes_magic_ward_hit_event_source);
impl_script_event_source_holder_event!(
    TESMoveAttachDetachEvent,
    tes_move_attach_detach_event_source
);
impl_script_event_source_holder_event!(TESObjectLoadedEvent, tes_object_loaded_event_source);
impl_script_event_source_holder_event!(
    TESObjectREFRTranslationEvent,
    tes_object_refr_translation_event_source
);
impl_script_event_source_holder_event!(TESOpenCloseEvent, tes_open_close_event_source);
impl_script_event_source_holder_event!(TESPackageEvent, tes_package_event_source);
impl_script_event_source_holder_event!(TESPerkEntryRunEvent, tes_perk_entry_run_event_source);
impl_script_event_source_holder_event!(TESQuestInitEvent, tes_quest_init_event_source);
impl_script_event_source_holder_event!(TESQuestStageEvent, tes_quest_stage_event_source);
impl_script_event_source_holder_event!(
    TESQuestStageItemDoneEvent,
    tes_quest_stage_item_done_event_source
);
impl_script_event_source_holder_event!(TESQuestStartStopEvent, tes_quest_start_stop_event_source);
impl_script_event_source_holder_event!(TESResetEvent, tes_reset_event_source);
impl_script_event_source_holder_event!(
    TESResolveNPCTemplatesEvent,
    tes_resolve_npc_templates_event_source
);
impl_script_event_source_holder_event!(TESSceneEvent, tes_scene_event_source);
impl_script_event_source_holder_event!(TESSceneActionEvent, tes_scene_action_event_source);
impl_script_event_source_holder_event!(TESScenePhaseEvent, tes_scene_phase_event_source);
impl_script_event_source_holder_event!(TESSellEvent, tes_sell_event_source);
impl_script_event_source_holder_event!(TESSleepStartEvent, tes_sleep_start_event_source);
impl_script_event_source_holder_event!(TESSleepStopEvent, tes_sleep_stop_event_source);
impl_script_event_source_holder_event!(TESSpellCastEvent, tes_spell_cast_event_source);
impl_script_event_source_holder_event!(TESPlayerBowShotEvent, tes_player_bow_shot_event_source);
impl_script_event_source_holder_event!(TESTopicInfoEvent, tes_topic_info_event_source);
impl_script_event_source_holder_event!(TESTrackedStatsEvent, tes_tracked_stats_event_source);
impl_script_event_source_holder_event!(TESTrapHitEvent, tes_trap_hit_event_source);
impl_script_event_source_holder_event!(TESTriggerEvent, tes_trigger_event_source);
impl_script_event_source_holder_event!(TESTriggerEnterEvent, tes_trigger_enter_event_source);
impl_script_event_source_holder_event!(TESTriggerLeaveEvent, tes_trigger_leave_event_source);
impl_script_event_source_holder_event!(TESUniqueIDChangeEvent, tes_unique_id_change_event_source);
impl_script_event_source_holder_event!(TESWaitStartEvent, tes_wait_start_event_source);
impl_script_event_source_holder_event!(TESWaitStopEvent, tes_wait_stop_event_source);
impl_script_event_source_holder_event!(
    TESSwitchRaceCompleteEvent,
    tes_switch_race_complete_event_source
);

impl ScriptEventSourceHolderEvent for TESFastTravelEndEvent {
    #[inline(always)]
    fn event_source(holder: &ScriptEventSourceHolder) -> *mut BSTEventSource<Self> {
        holder.tes_fast_travel_end_event_source()
    }
}

impl ScriptEventSourceHolder {
    crate::relocation_func! {
        pub fn get_singleton() -> *mut ScriptEventSourceHolder => crate::relocation::RelocationID::new(14108, 14298)
    }

    crate::runtime_optional_data_accessor! {
        fn tes_fast_travel_end_event_source_opt() -> BSTEventSource<TESFastTravelEndEvent> {
            se_ae: 0x1238,
            vr: 0,
        }
    }

    #[inline(always)]
    pub fn tes_fast_travel_end_event_source(&self) -> *mut BSTEventSource<TESFastTravelEndEvent> {
        self.tes_fast_travel_end_event_source_opt()
            .map(core::ptr::from_ref)
            .unwrap_or(core::ptr::null())
            .cast_mut()
    }

    #[inline(always)]
    pub fn get_event_source<T: ScriptEventSourceHolderEvent>(&self) -> *mut BSTEventSource<T> {
        T::event_source(self)
    }

    #[inline(always)]
    pub unsafe fn add_event_sink<T: ScriptEventSourceHolderEvent>(
        &mut self,
        sink: *mut BSTEventSink<T>,
    ) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).add_event_sink(sink) };
        }
    }

    #[inline(always)]
    pub unsafe fn prepend_event_sink<T: ScriptEventSourceHolderEvent>(
        &mut self,
        sink: *mut BSTEventSink<T>,
    ) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).prepend_event_sink(sink) };
        }
    }

    #[inline(always)]
    pub unsafe fn remove_event_sink<T: ScriptEventSourceHolderEvent>(
        &mut self,
        sink: *mut BSTEventSink<T>,
    ) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).remove_event_sink(sink) };
        }
    }

    #[inline(always)]
    pub unsafe fn send_event<T: ScriptEventSourceHolderEvent>(&mut self, event: *const T) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).send_event(event) };
        }
    }

    #[inline(always)]
    pub fn send_activate_event(
        &mut self,
        object_activated: NiPointer<TESObjectREFR>,
        action_ref: NiPointer<TESObjectREFR>,
    ) {
        let event = TESActivateEvent {
            object_activated,
            action_ref,
        };
        unsafe { self.send_event(&event) };
    }

    crate::relocation_func! {
        pub fn send_open_close_event(
            &mut self,
            ref_: &NiPointer<TESObjectREFR>,
            active_ref: &NiPointer<TESObjectREFR>,
            is_opened: bool
        ) => crate::relocation::RelocationID::new(14190, 14299)
    }

    #[inline(always)]
    pub fn send_spell_cast_event(&mut self, object: NiPointer<TESObjectREFR>, form_id: FormID) {
        let event = TESSpellCastEvent {
            object,
            spell: form_id,
            pad0c: 0,
        };
        unsafe { self.send_event(&event) };
    }
}
