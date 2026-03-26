use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SimpleAnimationGraphManagerHolder;
use crate::offsets::offsets_vtable::VTABLE_SimpleAnimationGraphManagerHolder;
use crate::re::{
    BSAnimationGraphManager, BSTSmartPointer, IAnimationGraphManagerHolder,
    SimpleAnimationGraphManagerLoadingTask,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SimpleAnimationGraphManagerHolder`
#[repr(C)]
pub struct SimpleAnimationGraphManagerHolder {
    pub base: IAnimationGraphManagerHolder, // 00
    pub animation_graph_manager: BSTSmartPointer<BSAnimationGraphManager>, // 08
    // TODO: CommonLib only forward-declares `SimpleAnimationGraphManagerLoadingTask` here in the
    // vendored tree, so its `NiRef` inheritance is not source-backed yet. Replace this raw
    // pointer stand-in with `NiPointer<SimpleAnimationGraphManagerLoadingTask>` once that header
    // or inheritance surface is translated honestly.
    pub loading_task: *mut SimpleAnimationGraphManagerLoadingTask, // 10
}

const _: () = assert!(core::mem::size_of::<SimpleAnimationGraphManagerHolder>() == 0x18);
const _: () = assert!(core::mem::offset_of!(SimpleAnimationGraphManagerHolder, base) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(SimpleAnimationGraphManagerHolder, animation_graph_manager) == 0x08
);
const _: () =
    assert!(core::mem::offset_of!(SimpleAnimationGraphManagerHolder, loading_task) == 0x10);

inherit!(SimpleAnimationGraphManagerHolder : IAnimationGraphManagerHolder);

impl RttiType for SimpleAnimationGraphManagerHolder {
    const RTTI: VariantID = RTTI_SimpleAnimationGraphManagerHolder;
}

impl SimpleAnimationGraphManagerHolder {
    pub const RTTI: VariantID = RTTI_SimpleAnimationGraphManagerHolder;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SimpleAnimationGraphManagerHolder;

    // override (IAnimationGraphManagerHolder)
    // 00 ~SimpleAnimationGraphManagerHolder
    // 02 GetAnimationGraphManagerImpl
    // 03 SetAnimationGraphManagerImpl
    // 05 ConstructAnimationGraph

    crate::virtual_method! {
        pub const VFUNC_UNK_13: usize = 0x13;
        pub fn unk_13()
    }
}

pub trait SimpleAnimationGraphManagerHolderExt {
    fn unk_13(&mut self);
}

impl<T: AsRef<SimpleAnimationGraphManagerHolder> + AsMut<SimpleAnimationGraphManagerHolder>>
    SimpleAnimationGraphManagerHolderExt for T
{
    #[inline(always)]
    fn unk_13(&mut self) {
        SimpleAnimationGraphManagerHolder::unk_13(self.as_mut())
    }
}
