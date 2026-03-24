use core_util::EnumSet;

use crate::offsets::offsets_rtti::RTTI_MagicItemDataCollector;
use crate::offsets::offsets_vtable::VTABLE_MagicItemDataCollector;
use crate::re::Effect;
use crate::re::FormType;
use crate::re::MagicItem;
use crate::re::MagicItemTraversalFunctor;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bst_array::BSTArray;
use crate::relocation::{RelocationID, RttiType, VariantID};
use core_util::inherit;

/// C++ `RE::MagicItemDataCollector::Flags`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicItemDataCollectorFlag {
    None = 0,
    SkipCostiest = 1 << 0,
    SkipProjectiles = 1 << 1,
    SkipArea = 1 << 2,
    OnlyFirstEffect = 1 << 3,
}

core_util::impl_enumset_type!(MagicItemDataCollectorFlag => u32);

/// C++ `RE::MagicItemDataCollector`
#[repr(C)]
pub struct MagicItemDataCollector {
    pub base: MagicItemTraversalFunctor,                 // 00
    pub projectile_effect_list: BSTArray<*mut Effect>,   // 10
    pub costliest_effect: *mut Effect,                   // 28
    pub max_cost: i32,                                   // 30
    pub pad34: u32,                                      // 34
    pub largest_area_effect: *mut Effect,                // 38
    pub highest_area: f32,                               // 40
    pub flags: EnumSet<MagicItemDataCollectorFlag, u32>, // 44
    pub summons_extra_large_creature: bool,              // 48
    pub pad49: u8,                                       // 49
    pub pad4a: u16,                                      // 4A
    pub pad4c: u32,                                      // 4C
}

const _: () = assert!(core::mem::size_of::<MagicItemDataCollector>() == 0x50);
const _: () =
    assert!(core::mem::offset_of!(MagicItemDataCollector, projectile_effect_list) == 0x10);
const _: () = assert!(core::mem::offset_of!(MagicItemDataCollector, costliest_effect) == 0x28);
const _: () = assert!(core::mem::offset_of!(MagicItemDataCollector, flags) == 0x44);

inherit!(MagicItemDataCollector : MagicItemTraversalFunctor);

impl RttiType for MagicItemDataCollector {
    const RTTI: VariantID = RTTI_MagicItemDataCollector;
}

impl MagicItemDataCollector {
    pub const RTTI: VariantID = RTTI_MagicItemDataCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicItemDataCollector;

    #[inline]
    pub fn new(magic_item: &MagicItem) -> Self {
        let only_first = magic_item.is(FormType::Ingredient);

        Self {
            base: MagicItemTraversalFunctor {
                vtable: Self::VTABLE[0].address() as *const usize,
                index: 0,
                pad0c: 0,
            },
            projectile_effect_list: BSTArray::new(),
            costliest_effect: core::ptr::null_mut(),
            max_cost: -1,
            pad34: 0,
            largest_area_effect: core::ptr::null_mut(),
            highest_area: 0.0,
            flags: if only_first {
                EnumSet::from_underlying(MagicItemDataCollectorFlag::OnlyFirstEffect as u32)
            } else {
                EnumSet::from_underlying(0)
            },
            summons_extra_large_creature: false,
            pad49: 0,
            pad4a: 0,
            pad4c: 0,
        }
    }

    crate::relocation_func! {
        pub fn call(&mut self, effect: *mut Effect) -> BSContainerForEachResult => RelocationID::new(33834, 34626)
    }
}
