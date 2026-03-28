use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SkyrimScript____MoveToFunctor;
use crate::offsets::offsets_vtable::VTABLE_SkyrimScript____MoveToFunctor;
use crate::re::{BSTSmartPointer, DelayFunctor, IVirtualMachine, NiPoint3, ObjectRefHandle};
use crate::relocation::RttiType;
use crate::relocation::VariantID;

/// C++ `RE::SkyrimScript::MoveToFunctor`
#[repr(C)]
pub struct MoveToFunctor {
    pub base: DelayFunctor,                   // 00
    pub source: ObjectRefHandle,              // 10
    pub destination: ObjectRefHandle,         // 14
    pub offset: NiPoint3,                     // 18
    pub rotation_offset: NiPoint3,            // 24
    pub match_rotation: bool,                 // 30
    pub exact_translate: bool,                // 31
    pub pad32: u16,                           // 32
    pub pad34: u32,                           // 34
    pub vm: BSTSmartPointer<IVirtualMachine>, // 38
}

const _: () = assert!(core::mem::size_of::<MoveToFunctor>() == 0x40);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, source) == 0x10);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, destination) == 0x14);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, offset) == 0x18);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, rotation_offset) == 0x24);
const _: () = assert!(core::mem::offset_of!(MoveToFunctor, vm) == 0x38);

inherit!(MoveToFunctor : DelayFunctor);

impl RttiType for MoveToFunctor {
    const RTTI: VariantID = RTTI_SkyrimScript____MoveToFunctor;
}

impl MoveToFunctor {
    pub const RTTI: VariantID = RTTI_SkyrimScript____MoveToFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SkyrimScript____MoveToFunctor;

    // override (DelayFunctor)
    // ~MoveToFunctor() override;                                                      // 00
    // RE::BSScript::Variable operator()() override;                                   // 01
    // bool IsLatent() const override;                                                 // 02
    // bool SaveImpl(BSStorage& a_storage) const override;                             // 04
    // FunctorType GetType() const override;                                           // 05
    // bool LoadImpl(const BSStorage& a_storage, std::uint32_t a_arg2, bool& a_arg3);  // 06
}
