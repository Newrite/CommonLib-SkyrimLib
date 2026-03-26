use crate::offsets::offsets_rtti::RTTI_DetectionListener;
use crate::offsets::offsets_vtable::VTABLE_DetectionListener;
use crate::re::NiRefObject;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

/// C++ `RE::DetectionListener`
#[repr(C)]
pub struct DetectionListener {
    pub base: NiRefObject, // 00
}

const _: () = assert!(core::mem::size_of::<DetectionListener>() == 0x10);

impl RttiType for DetectionListener {
    const RTTI: VariantID = RTTI_DetectionListener;
}

inherit!(DetectionListener : NiRefObject);

impl DetectionListener {
    pub const RTTI: VariantID = RTTI_DetectionListener;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DetectionListener;

    crate::virtual_method! {
        pub const VFUNC_UNK_02: usize = 0x02;
        pub fn unk_02(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x03;
        pub fn unk_03(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_04: usize = 0x04;
        pub fn unk_04(&mut self)
    }
}

pub trait DetectionListenerExt {
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
}

impl<T: AsMut<DetectionListener>> DetectionListenerExt for T {
    #[inline(always)]
    fn unk_02(&mut self) {
        DetectionListener::unk_02(self.as_mut())
    }

    #[inline(always)]
    fn unk_03(&mut self) {
        DetectionListener::unk_03(self.as_mut())
    }

    #[inline(always)]
    fn unk_04(&mut self) {
        DetectionListener::unk_04(self.as_mut())
    }
}
