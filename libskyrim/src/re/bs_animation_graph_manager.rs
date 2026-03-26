use crate::offsets::offsets_rtti::RTTI_BSAnimationGraphManager;
use crate::offsets::offsets_vtable::VTABLE_BSAnimationGraphManager;
use crate::re::bs_atomic::BSSpinLock;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    BSAnimationGraphChannel, BSAnimationGraphEvent, BSEventNotifyControl, BSFixedString,
    BSIntrusiveRefCounted, BSScrapArray, BSTArray, BSTEventSink, BSTEventSource, BSTSmallArray,
    BSTSmartPointer, BShkbAnimationGraph, hkbVariableValue,
};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::version::RUNTIME_SSE_1_6_629;

pub type BSAnimationGraphManagerPtr = BSTSmartPointer<BSAnimationGraphManager>;

/// C++ `RE::AnimVariableCacheInfo`
#[repr(C)]
pub struct AnimVariableCacheInfo {
    pub variable_name: BSFixedString,    // 00
    pub variable: *mut hkbVariableValue, // 08
}

const _: () = assert!(core::mem::size_of::<AnimVariableCacheInfo>() == 0x10);
const _: () = assert!(core::mem::offset_of!(AnimVariableCacheInfo, variable_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(AnimVariableCacheInfo, variable) == 0x08);

/// Honest common prefix of C++ `RE::BSAnimationGraphVariableCache`.
#[repr(C)]
pub struct BSAnimationGraphVariableCache {
    pub variable_cache: BSTArray<AnimVariableCacheInfo>, // 00
    pub update_lock: BSSpinLock,                         // 18
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphVariableCache>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphVariableCache, variable_cache) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphVariableCache, update_lock) == 0x18);

impl BSAnimationGraphVariableCache {
    crate::runtime_optional_data_accessor! {
        pub fn graph_lock() -> BSSpinLock {
            version: RUNTIME_SSE_1_6_629,
            older: 0x00,
            newer: 0x20
        }
    }

    crate::runtime_data_accessor! {
        pub fn animation_graph() -> BSTSmartPointer<BShkbAnimationGraph> {
            version: RUNTIME_SSE_1_6_629,
            older: 0x20,
            newer: 0x28
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn animation_graph_mut() -> BSTSmartPointer<BShkbAnimationGraph> {
            version: RUNTIME_SSE_1_6_629,
            older: 0x20,
            newer: 0x28
        }
    }
}

/// C++ `RE::BSAnimationGraphManager::AnimationVariable::Value`
#[repr(C)]
#[derive(Clone, Copy)]
pub union BSAnimationGraphManagerAnimationVariableValue {
    pub b: bool,
    pub i: i32,
    pub f: f32,
}

const _: () =
    assert!(core::mem::size_of::<BSAnimationGraphManagerAnimationVariableValue>() == 0x04);

/// C++ `RE::BSAnimationGraphManager::AnimationVariable`
#[repr(C)]
pub struct BSAnimationGraphManagerAnimationVariable {
    pub name: BSFixedString,                                       // 00
    pub value: *mut BSAnimationGraphManagerAnimationVariableValue, // 08
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphManagerAnimationVariable>() == 0x10);

/// C++ `RE::BSAnimationGraphManager::ClipData`
#[repr(C)]
pub struct BSAnimationGraphManagerClipData {
    pub clip_name: BSFixedString, // 00
    pub time_scale: f32,          // 08
    pub field_c: f32,             // 0C
    pub pos_scale: f32,           // 10
    pub field_14: f32,            // 14
    pub x_neg: bool,              // 18
    pub pad19: [u8; 7],           // 19
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphManagerClipData>() == 0x20);

/// C++ `RE::BSAnimationGraphManager::RUNTIME_DATA`
#[repr(C)]
pub struct BSAnimationGraphManagerRuntimeData {
    pub update_lock: BSSpinLock,            // 00
    pub dependent_manager_lock: BSSpinLock, // 08
    pub active_graph: u32,                  // 10
    pub generate_depth: u32,                // 14
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphManagerRuntimeData>() == 0x18);
const _: () =
    assert!(core::mem::offset_of!(BSAnimationGraphManagerRuntimeData, update_lock) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(BSAnimationGraphManagerRuntimeData, dependent_manager_lock) == 0x08
);
const _: () =
    assert!(core::mem::offset_of!(BSAnimationGraphManagerRuntimeData, active_graph) == 0x10);
const _: () =
    assert!(core::mem::offset_of!(BSAnimationGraphManagerRuntimeData, generate_depth) == 0x14);

/// Honest common prefix of C++ `RE::BSAnimationGraphManager`.
#[repr(C)]
pub struct BSAnimationGraphManager {
    pub event_sink: BSTEventSink<BSAnimationGraphEvent>, // 00
    pub intrusive_ref_counted: BSIntrusiveRefCounted,    // 08
    pub pad0c: u32,                                      // 0C
    pub bound_channels: BSTArray<BSTSmartPointer<BSAnimationGraphChannel>>, // 10
    pub bumped_channels: BSTArray<BSTSmartPointer<BSAnimationGraphChannel>>, // 28
    pub graphs: BSTSmallArray<BSTSmartPointer<BShkbAnimationGraph>, 8>, // 40
    pub sub_managers: BSTArray<BSAnimationGraphManagerPtr>, // 58
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphManager>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphManager, event_sink) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSAnimationGraphManager, intrusive_ref_counted) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphManager, bound_channels) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphManager, bumped_channels) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphManager, graphs) == 0x40);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphManager, sub_managers) == 0x58);

impl RttiType for BSAnimationGraphManager {
    const RTTI: VariantID = RTTI_BSAnimationGraphManager;
}

impl AsRef<BSAnimationGraphManager> for BSAnimationGraphManager {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSAnimationGraphManager> for BSAnimationGraphManager {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSAnimationGraphManager {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.intrusive_ref_counted.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.intrusive_ref_counted.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BSAnimationGraphManager {
    pub const RTTI: VariantID = RTTI_BSAnimationGraphManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSAnimationGraphManager;

    // ~BSAnimationGraphManager() override;  // 00

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    // override (BSTEventSink<BSAnimationGraphEvent>)
    crate::virtual_method! {
        pub const VFUNC_PROCESS_EVENT: usize = 0x01;
        pub fn process_event(
            &mut self,
            event: *const BSAnimationGraphEvent,
            event_source: *mut BSTEventSource<BSAnimationGraphEvent>
        ) -> BSEventNotifyControl
    }

    crate::runtime_data_accessor! {
        pub fn variable_cache() -> BSAnimationGraphVariableCache {
            offset: 0x70
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn variable_cache_mut() -> BSAnimationGraphVariableCache {
            offset: 0x70
        }
    }

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> BSAnimationGraphManagerRuntimeData {
            version: RUNTIME_SSE_1_6_629,
            older: 0x98,
            newer: 0xA0
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> BSAnimationGraphManagerRuntimeData {
            version: RUNTIME_SSE_1_6_629,
            older: 0x98,
            newer: 0xA0
        }
    }

    crate::relocation_func! {
        pub fn query_animations_by_events(
            &mut self,
            events: &BSScrapArray<BSFixedString>,
            active_graph_index: i32,
            project_name: &mut BSFixedString,
            clips: &mut BSScrapArray<BSAnimationGraphManagerClipData>
        ) -> bool => RelocationID::new(62432, 0)
    }

    crate::relocation_func! {
        pub fn query_animations_from_time(
            &mut self,
            from_time: f32,
            project_name: &mut BSFixedString,
            clips: &mut BSScrapArray<BSAnimationGraphManagerClipData>,
            active_graph_index: i32
        ) -> bool => RelocationID::new(62431, 0)
    }
}
