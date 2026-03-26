use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_PlayerControls;
use crate::offsets::offsets_vtable::VTABLE_PlayerControls;
use crate::re::{
    ActivateHandler, ActorHandle, AttackBlockHandler, AutoMoveHandler, BSSpinLock, BSTArray,
    BSTEventSink, BSTSingletonSDM, InputEvent, JumpHandler, LookHandler, MenuModeChangeEvent,
    MenuOpenCloseEvent, MovementHandler, PlayerControlsData, PlayerInputHandler,
    ReadyWeaponHandler, RunHandler, ShoutHandler, SneakHandler, SprintHandler, TESFurnitureEvent,
    TogglePOVHandler, ToggleRunHandler,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::PlayerControls`
#[repr(C)]
pub struct PlayerControls {
    pub input_event_sink: BSTEventSink<*mut InputEvent>, // 000
    pub menu_open_close_event_sink: BSTEventSink<MenuOpenCloseEvent>, // 008
    pub menu_mode_change_event_sink: BSTEventSink<MenuModeChangeEvent>, // 010
    pub furniture_event_sink: BSTEventSink<TESFurnitureEvent>, // 018
    pub singleton: BSTSingletonSDM<PlayerControls>,      // 020
    pub pad021: u8,                                      // 021
    pub pad022: u16,                                     // 022
    pub data: PlayerControlsData,                        // 024
    pub pad054: u32,                                     // 054
    pub handlers: BSTArray<*mut PlayerInputHandler>,     // 058
    pub unk070: BSTArray<*mut c_void>,                   // 070
    pub unk088: BSTArray<*mut c_void>,                   // 088
    pub unk0a0: [u8; 8],                                 // 0A0
    pub unk0a8: u64,                                     // 0A8
    pub unk0b0: [f32; 8],                                // 0B0
    pub unk0d0: [u32; 10],                               // 0D0
    pub unk0f8: [u8; 8],                                 // 0F8
    pub unk100: [f32; 20],                               // 100
    pub action_interested_actor: BSTArray<ActorHandle>,  // 150
    pub actor_array_lock: BSSpinLock,                    // 168
    pub movement_handler: *mut MovementHandler,          // 170
    pub look_handler: *mut LookHandler,                  // 178
    pub sprint_handler: *mut SprintHandler,              // 180
    pub ready_weapon_handler: *mut ReadyWeaponHandler,   // 188
    pub auto_move_handler: *mut AutoMoveHandler,         // 190
    pub toggle_run_handler: *mut ToggleRunHandler,       // 198
    pub activate_handler: *mut ActivateHandler,          // 1A0
    pub jump_handler: *mut JumpHandler,                  // 1A8
    pub shout_handler: *mut ShoutHandler,                // 1B0
    pub attack_block_handler: *mut AttackBlockHandler,   // 1B8
    pub run_handler: *mut RunHandler,                    // 1C0
    pub sneak_handler: *mut SneakHandler,                // 1C8
    pub toggle_pov_handler: *mut TogglePOVHandler,       // 1D0
    pub notifying_handlers: bool,                        // 1D8
    pub block_player_input: bool,                        // 1D9
    pub unk1da: u16,                                     // 1DA
    pub unk1dc: u32,                                     // 1DC
}

const _: () = assert!(core::mem::size_of::<PlayerControls>() == 0x1E0);
const _: () = assert!(core::mem::offset_of!(PlayerControls, input_event_sink) == 0x000);
const _: () = assert!(core::mem::offset_of!(PlayerControls, menu_open_close_event_sink) == 0x008);
const _: () = assert!(core::mem::offset_of!(PlayerControls, menu_mode_change_event_sink) == 0x010);
const _: () = assert!(core::mem::offset_of!(PlayerControls, furniture_event_sink) == 0x018);
const _: () = assert!(core::mem::offset_of!(PlayerControls, singleton) == 0x020);
const _: () = assert!(core::mem::offset_of!(PlayerControls, data) == 0x024);
const _: () = assert!(core::mem::offset_of!(PlayerControls, handlers) == 0x058);
const _: () = assert!(core::mem::offset_of!(PlayerControls, action_interested_actor) == 0x150);
const _: () = assert!(core::mem::offset_of!(PlayerControls, actor_array_lock) == 0x168);
const _: () = assert!(core::mem::offset_of!(PlayerControls, movement_handler) == 0x170);
const _: () = assert!(core::mem::offset_of!(PlayerControls, activate_handler) == 0x1A0);
const _: () = assert!(core::mem::offset_of!(PlayerControls, run_handler) == 0x1C0);
const _: () = assert!(core::mem::offset_of!(PlayerControls, toggle_pov_handler) == 0x1D0);
const _: () = assert!(core::mem::offset_of!(PlayerControls, notifying_handlers) == 0x1D8);
const _: () = assert!(core::mem::offset_of!(PlayerControls, block_player_input) == 0x1D9);

impl RttiType for PlayerControls {
    const RTTI: VariantID = RTTI_PlayerControls;
}

inherit!(PlayerControls => BSTEventSink<*mut InputEvent>, input_event_sink);
inherit!(
    PlayerControls => BSTEventSink<MenuOpenCloseEvent>,
    menu_open_close_event_sink
);
inherit!(
    PlayerControls => BSTEventSink<MenuModeChangeEvent>,
    menu_mode_change_event_sink
);
inherit!(PlayerControls => BSTEventSink<TESFurnitureEvent>, furniture_event_sink);
inherit!(PlayerControls => BSTSingletonSDM<PlayerControls>, singleton);

impl PlayerControls {
    pub const RTTI: VariantID = RTTI_PlayerControls;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerControls;

    // override (BSTEventSink<InputEvent*>)
    // BSEventNotifyControl ProcessEvent(...) override;                 // 01

    // override (BSTEventSink<MenuOpenCloseEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;                 // 01

    // override (BSTEventSink<MenuModeChangeEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;                 // 01

    // override (BSTEventSink<TESFurnitureEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;                 // 01

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut PlayerControls => RelocationID::new(514706, 400864)
    }

    crate::relocation_func! {
        fn ctor_impl(this: *mut PlayerControls) -> *mut PlayerControls => RelocationID::new(41257, 42336)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut PlayerControls {
        *Self::singleton_ptr()
    }

    #[inline(always)]
    pub fn ctor(&mut self) -> *mut PlayerControls {
        Self::ctor_impl(self)
    }

    #[inline(always)]
    pub const fn get_activate_handler(&self) -> *mut ActivateHandler {
        self.activate_handler
    }
}
