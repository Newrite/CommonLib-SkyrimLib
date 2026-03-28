use core::ffi::c_void;

use crate::re::{
    BSFixedString, BSSpinLock, BSTArray, BSTEventSink, BSTEventSource, BSTHashMap, BSTSingletonSDM,
    BSTTuple, BSTimer, GFxMovieView, GPtr, IMenu, MenuModeChangeEvent, MenuOpenCloseEvent,
};

pub type UICreate_t = unsafe extern "C" fn() -> *mut IMenu;

/// C++ `RE::UI::UIMenuEntry`
#[repr(C)]
pub struct UIMenuEntry {
    pub menu: GPtr<IMenu>,          // 00
    pub create: Option<UICreate_t>, // 08
}

const _: () = assert!(core::mem::size_of::<UIMenuEntry>() == 0x10);
const _: () = assert!(core::mem::offset_of!(UIMenuEntry, menu) == 0x0);
const _: () = assert!(core::mem::offset_of!(UIMenuEntry, create) == 0x8);

/// C++ `RE::UI` VR-only tail.
#[repr(C)]
pub struct UIVRRuntimeData {
    pub unk1c8: u32, // 00
    // TODO: `UI.h` comments this second field as `unk1CA // 1CA`, but the
    // class `STATIC_ASSERT_SIZE(UI, ..., 0x1D0, 0x1D0)` makes the aligned
    // offset `0x1CC`. Keep the honest aligned layout here until upstream
    // source proves a different contract.
    pub unk1cc: u32, // 04
}

const _: () = assert!(core::mem::size_of::<UIVRRuntimeData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(UIVRRuntimeData, unk1c8) == 0x0);
const _: () = assert!(core::mem::offset_of!(UIVRRuntimeData, unk1cc) == 0x4);

/// Honest common prefix of C++ `RE::UI`.
#[repr(C)]
pub struct UI {
    pub base: BSTSingletonSDM<UI>, // 000
    pub pad001: [u8; 7],           // 001
    pub menu_open_close_event_source: BSTEventSource<MenuOpenCloseEvent>, // 008
    pub menu_mode_change_event_source: BSTEventSource<MenuModeChangeEvent>, // 060
    pub unknown_event_source: BSTEventSource<*mut c_void>, // 0B8
    pub menu_stack: BSTArray<GPtr<IMenu>>, // 110
    pub menu_map: BSTHashMap<BSFixedString, UIMenuEntry>, // 128
    pub process_messages_lock: BSSpinLock, // 158
    pub num_pauses_game: u32,      // 160
    pub num_item_menus: u32,       // 164
    pub num_disable_pause_menu: u32, // 168
    pub num_allow_saving: u32,     // 16C
    pub num_dont_hide_cursor_when_topmost: u32, // 170
    pub num_custom_rendering: u32, // 174
    pub num_application_menus: u32, // 178
    pub modal: bool,               // 17C
    pub pad17d: u8,                // 17D
    pub pad17e: u16,               // 17E
    pub ui_timer: BSTimer,         // 180
    pub menu_system_visible: bool, // 1C0
    pub closing_all_menus: bool,   // 1C1
    pub pad1c2: u16,               // 1C2
    pub pad1c4: u32,               // 1C4
}

const _: () = assert!(core::mem::size_of::<UI>() == 0x1C8);
const _: () = assert!(core::mem::offset_of!(UI, menu_open_close_event_source) == 0x008);
const _: () = assert!(core::mem::offset_of!(UI, menu_mode_change_event_source) == 0x060);
const _: () = assert!(core::mem::offset_of!(UI, unknown_event_source) == 0x0B8);
const _: () = assert!(core::mem::offset_of!(UI, menu_stack) == 0x110);
const _: () = assert!(core::mem::offset_of!(UI, menu_map) == 0x128);
const _: () = assert!(core::mem::offset_of!(UI, process_messages_lock) == 0x158);
const _: () = assert!(core::mem::offset_of!(UI, ui_timer) == 0x180);
const _: () = assert!(core::mem::offset_of!(UI, menu_system_visible) == 0x1C0);

pub trait UIEventSourceEvent: Sized {
    fn event_source(ui: &UI) -> *mut BSTEventSource<Self>;
}

impl UIEventSourceEvent for MenuOpenCloseEvent {
    #[inline(always)]
    fn event_source(ui: &UI) -> *mut BSTEventSource<Self> {
        core::ptr::from_ref(&ui.menu_open_close_event_source).cast_mut()
    }
}

impl UIEventSourceEvent for MenuModeChangeEvent {
    #[inline(always)]
    fn event_source(ui: &UI) -> *mut BSTEventSource<Self> {
        core::ptr::from_ref(&ui.menu_mode_change_event_source).cast_mut()
    }
}

impl UIEventSourceEvent for *mut c_void {
    #[inline(always)]
    fn event_source(ui: &UI) -> *mut BSTEventSource<Self> {
        core::ptr::from_ref(&ui.unknown_event_source).cast_mut()
    }
}

impl UI {
    pub const VR_RUNTIME_DATA_OFFSET: usize = 0x1C8;

    crate::relocation_variable! {
        fn singleton() -> *mut UI => crate::relocation::RelocationID::new(514178, 400327), is_ptr
    }

    crate::relocation_func! {
        pub fn get_top_most_menu(&mut self, result: &mut *mut IMenu, depth_limit: u32)
            => crate::relocation::RelocationID::new(79944, 82081)
    }

    crate::runtime_optional_data_accessor! {
        pub fn get_vr_runtime_data() -> UIVRRuntimeData {
            se_ae: 0,
            vr: Self::VR_RUNTIME_DATA_OFFSET,
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn get_vr_runtime_data_mut() -> UIVRRuntimeData {
            se_ae: 0,
            vr: Self::VR_RUNTIME_DATA_OFFSET,
        }
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut UI {
        Self::singleton()
    }

    #[inline(always)]
    pub fn get_event_source<T: UIEventSourceEvent>(&self) -> *mut BSTEventSource<T> {
        T::event_source(self)
    }

    /// # Safety
    /// `sink` must remain valid for any future retained dispatches from the
    /// underlying `BSTEventSource<T>`.
    #[inline(always)]
    pub unsafe fn add_event_sink<T: UIEventSourceEvent>(&mut self, sink: *mut BSTEventSink<T>) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).add_event_sink(sink) };
        }
    }

    /// # Safety
    /// `sink` must have been registered on this event source and remain valid
    /// for the duration of the removal call.
    #[inline(always)]
    pub unsafe fn remove_event_sink<T: UIEventSourceEvent>(&mut self, sink: *mut BSTEventSink<T>) {
        let source = self.get_event_source::<T>();
        debug_assert!(!source.is_null());
        if !source.is_null() {
            unsafe { (*source).remove_event_sink(sink) };
        }
    }

    #[inline(always)]
    pub fn game_is_paused(&self) -> bool {
        self.num_pauses_game > 0
    }

    pub fn get_menu(&self, menu_name: &str) -> GPtr<IMenu> {
        let key = BSFixedString::from_str(menu_name);
        let entry = self.menu_map.find(&key);
        if entry.is_null() {
            GPtr::null()
        } else {
            unsafe { (*entry).second.menu.clone() }
        }
    }

    pub fn get_movie_view(&self, menu_name: &str) -> GPtr<GFxMovieView> {
        let menu = self.get_menu(menu_name);
        if menu.is_null() {
            GPtr::null()
        } else {
            unsafe { (*menu.as_ptr()).ui_movie.clone() }
        }
    }

    #[inline(always)]
    pub fn is_application_menu_open(&self) -> bool {
        self.num_application_menus > 0
    }

    #[inline(always)]
    pub fn is_cursor_hidden_when_topmost(&self) -> bool {
        self.num_dont_hide_cursor_when_topmost == 0
    }

    #[inline(always)]
    pub fn is_item_menu_open(&self) -> bool {
        self.num_item_menus > 0
    }

    #[inline(always)]
    pub fn is_menu_open(&self, menu_name: &str) -> bool {
        let menu = self.get_menu(menu_name);
        !menu.is_null() && unsafe { (*menu.as_ptr()).on_stack() }
    }

    #[inline(always)]
    pub fn is_modal_menu_open(&self) -> bool {
        self.modal
    }

    #[inline(always)]
    pub fn is_pause_menu_disabled(&self) -> bool {
        self.num_disable_pause_menu > 0
    }

    #[inline(always)]
    pub fn is_saving_allowed(&self) -> bool {
        self.num_allow_saving > 0
    }

    #[inline(always)]
    pub fn is_showing_menus(&self) -> bool {
        self.menu_system_visible
    }

    #[inline(always)]
    pub fn is_using_custom_rendering(&self) -> bool {
        self.num_custom_rendering > 0
    }

    pub fn register(&mut self, menu_name: &str, creator: Option<UICreate_t>) {
        let value = BSTTuple::new(
            BSFixedString::from_str(menu_name),
            UIMenuEntry {
                menu: GPtr::null(),
                create: creator,
            },
        );
        unsafe {
            self.menu_map.insert(value);
        }
    }

    #[inline(always)]
    pub fn show_menus(&mut self, show: bool) {
        self.menu_system_visible = show;
    }
}
