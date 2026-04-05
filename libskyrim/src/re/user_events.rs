#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::re::BSFixedString;
use crate::re::bst_singleton::BSTSingletonSDM;
use crate::relocation::RelocationID;

/// C++ named layer `RE::UserEvents::INPUT_CONTEXT_IDS`
pub struct INPUT_CONTEXT_IDS;

/// Runtime-aware translation of C++ `RE::UserEvents::INPUT_CONTEXT_ID`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INPUT_CONTEXT_ID(pub u32);

impl INPUT_CONTEXT_ID {
    pub const kGameplay: Self = Self(0);
    pub const kMenuMode: Self = Self(1);
    pub const kConsole: Self = Self(2);
    pub const kItemMenu: Self = Self(3);
    pub const kInventory: Self = Self(4);
    pub const kDebugText: Self = Self(5);
    pub const kFavorites: Self = Self(6);
    pub const kMap: Self = Self(7);
    pub const kStats: Self = Self(8);
    pub const kCursor: Self = Self(9);
    pub const kBook: Self = Self(10);
    pub const kDebugOverlay: Self = Self(11);
    pub const kJournal: Self = Self(12);
    pub const kTFCMode: Self = Self(13);
    pub const kMapDebug: Self = Self(14);
    pub const kLockpicking: Self = Self(15);

    #[inline(always)]
    pub fn kMarketplace() -> Option<Self> {
        crate::runtime::is_ae().then_some(Self(16))
    }

    #[inline(always)]
    pub fn kFavor() -> Self {
        Self(crate::runtime::relocate_all(16, 17, 16))
    }

    #[inline(always)]
    pub const fn kAETotal() -> u32 {
        18
    }

    #[inline(always)]
    pub const fn kVRTotal() -> u32 {
        17
    }

    #[inline(always)]
    pub fn kTotal() -> u32 {
        crate::runtime::relocate_all(17, 18, 17)
    }

    #[inline(always)]
    pub fn kNone() -> Self {
        Self(crate::runtime::relocate_all(18, 19, 22))
    }

    #[inline(always)]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl crate::rex::EnumSetType<u32> for INPUT_CONTEXT_ID {
    #[inline(always)]
    fn to_underlying(self) -> u32 {
        self.get()
    }
}

impl TryFrom<u32> for INPUT_CONTEXT_ID {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl INPUT_CONTEXT_IDS {
    pub const kGameplay: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kGameplay;
    pub const kMenuMode: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kMenuMode;
    pub const kConsole: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kConsole;
    pub const kItemMenu: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kItemMenu;
    pub const kInventory: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kInventory;
    pub const kDebugText: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kDebugText;
    pub const kFavorites: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kFavorites;
    pub const kMap: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kMap;
    pub const kStats: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kStats;
    pub const kCursor: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kCursor;
    pub const kBook: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kBook;
    pub const kDebugOverlay: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kDebugOverlay;
    pub const kJournal: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kJournal;
    pub const kTFCMode: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kTFCMode;
    pub const kMapDebug: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kMapDebug;
    pub const kLockpicking: INPUT_CONTEXT_ID = INPUT_CONTEXT_ID::kLockpicking;

    #[inline(always)]
    pub fn kMarketplace() -> Option<INPUT_CONTEXT_ID> {
        INPUT_CONTEXT_ID::kMarketplace()
    }

    #[inline(always)]
    pub fn kFavor() -> INPUT_CONTEXT_ID {
        INPUT_CONTEXT_ID::kFavor()
    }

    #[inline(always)]
    pub const fn kAETotal() -> u32 {
        INPUT_CONTEXT_ID::kAETotal()
    }

    #[inline(always)]
    pub const fn kVRTotal() -> u32 {
        INPUT_CONTEXT_ID::kVRTotal()
    }

    #[inline(always)]
    pub fn kTotal() -> u32 {
        INPUT_CONTEXT_ID::kTotal()
    }

    #[inline(always)]
    pub fn kNone() -> INPUT_CONTEXT_ID {
        INPUT_CONTEXT_ID::kNone()
    }
}

impl From<INPUT_CONTEXT_ID> for usize {
    #[inline(always)]
    fn from(value: INPUT_CONTEXT_ID) -> Self {
        value.get() as usize
    }
}

/// C++ `RE::UserEvents::USER_EVENT_FLAG`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum USER_EVENT_FLAG {
    kNone = 0,
    kMovement = 1 << 0,
    kLooking = 1 << 1,
    kActivate = 1 << 2,
    kMenu = 1 << 3,
    kConsole = 1 << 4,
    kPOVSwitch = 1 << 5,
    kFighting = 1 << 6,
    kSneaking = 1 << 7,
    kMainFour = 1 << 8,
    kWheelZoom = 1 << 9,
    kJumping = 1 << 10,
    kVATS = 1 << 11,
    kInvalid = 1 << 31,
    kAll = u32::MAX,
}

core_util::impl_enumset_type!(USER_EVENT_FLAG => u32);

/// C++ `RE::UserEvents`
#[repr(C)]
pub struct UserEvents {
    pub base: BSTSingletonSDM<UserEvents>,  // 000
    pub pad001: [u8; 7],                    // 001
    pub forward: BSFixedString,             // 008
    pub back: BSFixedString,                // 010
    pub strafe_left: BSFixedString,         // 018
    pub strafe_right: BSFixedString,        // 020
    pub move_: BSFixedString,               // 028
    pub look: BSFixedString,                // 030
    pub activate: BSFixedString,            // 038
    pub left_attack: BSFixedString,         // 040
    pub right_attack: BSFixedString,        // 048
    pub dual_attack: BSFixedString,         // 050
    pub force_release: BSFixedString,       // 058
    pub pause: BSFixedString,               // 060
    pub ready_weapon: BSFixedString,        // 068
    pub toggle_pov: BSFixedString,          // 070
    pub jump: BSFixedString,                // 078
    pub journal: BSFixedString,             // 080
    pub sprint: BSFixedString,              // 088
    pub sneak: BSFixedString,               // 090
    pub shout: BSFixedString,               // 098
    pub kinect_shout: BSFixedString,        // 0A0
    pub grab: BSFixedString,                // 0A8
    pub run: BSFixedString,                 // 0B0
    pub toggle_run: BSFixedString,          // 0B8
    pub auto_move: BSFixedString,           // 0C0
    pub quicksave: BSFixedString,           // 0C8
    pub quickload: BSFixedString,           // 0D0
    pub new_save: BSFixedString,            // 0D8
    pub inventory: BSFixedString,           // 0E0
    pub stats: BSFixedString,               // 0E8
    pub map: BSFixedString,                 // 0F0
    pub screenshot: BSFixedString,          // 0F8
    pub multi_screenshot: BSFixedString,    // 100
    pub console: BSFixedString,             // 108
    pub camera_path: BSFixedString,         // 110
    pub tween_menu: BSFixedString,          // 118
    pub take_all: BSFixedString,            // 120
    pub accept: BSFixedString,              // 128
    pub cancel: BSFixedString,              // 130
    pub up: BSFixedString,                  // 138
    pub down: BSFixedString,                // 140
    pub left: BSFixedString,                // 148
    pub right: BSFixedString,               // 150
    pub page_up: BSFixedString,             // 158
    pub page_down: BSFixedString,           // 160
    pub pick: BSFixedString,                // 168
    pub pick_next: BSFixedString,           // 170
    pub pick_previous: BSFixedString,       // 178
    pub cursor: BSFixedString,              // 180
    pub kinect: BSFixedString,              // 188
    pub sprint_start: BSFixedString,        // 190
    pub sprint_stop: BSFixedString,         // 198
    pub sneak_start: BSFixedString,         // 1A0
    pub sneak_stop: BSFixedString,          // 1A8
    pub block_start: BSFixedString,         // 1B0
    pub block_stop: BSFixedString,          // 1B8
    pub block_bash: BSFixedString,          // 1C0
    pub attack_start: BSFixedString,        // 1C8
    pub attack_power_start: BSFixedString,  // 1D0
    pub reverse_direction: BSFixedString,   // 1D8
    pub unequip: BSFixedString,             // 1E0
    pub zoom_in: BSFixedString,             // 1E8
    pub zoom_out: BSFixedString,            // 1F0
    pub rotate_item: BSFixedString,         // 1F8
    pub left_stick: BSFixedString,          // 200
    pub prev_page: BSFixedString,           // 208
    pub next_page: BSFixedString,           // 210
    pub prev_sub_page: BSFixedString,       // 218
    pub next_sub_page: BSFixedString,       // 220
    pub left_equip: BSFixedString,          // 228
    pub right_equip: BSFixedString,         // 230
    pub toggle_favorite: BSFixedString,     // 238
    pub favorites: BSFixedString,           // 240
    pub hotkey1: BSFixedString,             // 248
    pub hotkey2: BSFixedString,             // 250
    pub hotkey3: BSFixedString,             // 258
    pub hotkey4: BSFixedString,             // 260
    pub hotkey5: BSFixedString,             // 268
    pub hotkey6: BSFixedString,             // 270
    pub hotkey7: BSFixedString,             // 278
    pub hotkey8: BSFixedString,             // 280
    pub quick_inventory: BSFixedString,     // 288
    pub quick_magic: BSFixedString,         // 290
    pub quick_stats: BSFixedString,         // 298
    pub quick_map: BSFixedString,           // 2A0
    pub toggle_cursor: BSFixedString,       // 2A8
    pub wait: BSFixedString,                // 2B0
    pub click: BSFixedString,               // 2B8
    pub map_look_mode: BSFixedString,       // 2C0
    pub equip: BSFixedString,               // 2C8
    pub drop_item: BSFixedString,           // 2D0
    pub rotate: BSFixedString,              // 2D8
    pub next_focus: BSFixedString,          // 2E0
    pub prev_focus: BSFixedString,          // 2E8
    pub set_active_quest: BSFixedString,    // 2F0
    pub place_player_marker: BSFixedString, // 2F8
    pub x_button: BSFixedString,            // 300
    pub y_button: BSFixedString,            // 308
    pub charge_item: BSFixedString,         // 310
    pub unk318: BSFixedString,              // 318
    pub player_position: BSFixedString,     // 320
    pub local_map: BSFixedString,           // 328
    pub local_map_move_mode: BSFixedString, // 330
    pub item_zoom: BSFixedString,           // 338
}

const _: () = assert!(core::mem::size_of::<UserEvents>() == 0x340);
const _: () = assert!(core::mem::offset_of!(UserEvents, forward) == 0x008);
const _: () = assert!(core::mem::offset_of!(UserEvents, quick_inventory) == 0x288);
const _: () = assert!(core::mem::offset_of!(UserEvents, item_zoom) == 0x338);

impl UserEvents {
    crate::relocation_variable! {
        fn singleton() -> *mut UserEvents => RelocationID::new(516458, 402638), is_indirect_ptr
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut UserEvents {
        Self::singleton()
    }
}
