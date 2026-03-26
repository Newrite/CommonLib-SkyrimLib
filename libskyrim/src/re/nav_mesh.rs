use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_NavMesh;
use crate::offsets::offsets_vtable::VTABLE_NavMesh;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{BSNavmeshVertex, BSTArray, FormCastable, FormType, TESChildCell, TESForm};
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
struct NavMeshRefCountView {
    pad00: [u8; 0x28],
    base: BSIntrusiveRefCounted,
}

const _: () = assert!(core::mem::size_of::<NavMeshRefCountView>() == 0x2C);
const _: () = assert!(core::mem::offset_of!(NavMeshRefCountView, base) == 0x28);

/// Partial translation of `RE::NavMesh`.
///
/// The prefix is enough for current `TESObjectREFR`/`TESObjectCELL` helpers
/// without keeping consumer-local `*View` stand-ins.
#[repr(C)]
pub struct NavMesh {
    pub base: TESForm,                       // 00
    pub child_cell: TESChildCell,            // 20
    pub pad28: [u8; 0x10],                   // 28
    pub vertices: BSTArray<BSNavmeshVertex>, // 38
}

const _: () = assert!(core::mem::size_of::<NavMesh>() == 0x50);
const _: () = assert!(core::mem::offset_of!(NavMesh, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NavMesh, child_cell) == 0x20);
const _: () = assert!(core::mem::offset_of!(NavMesh, vertices) == 0x38);

impl RttiType for NavMesh {
    const RTTI: VariantID = RTTI_NavMesh;
}

impl FormCastable for NavMesh {
    const TARGET_FORM_TYPE: FormType = FormType::NavMesh;
}

inherit!(NavMesh : TESForm);

impl BSTSmartPointerIntrusiveRefCountable for NavMesh {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        unsafe {
            (&*(self as *const Self as *const NavMeshRefCountView))
                .base
                .inc_ref()
        };
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        unsafe {
            (&*(self as *const Self as *const NavMeshRefCountView))
                .base
                .dec_ref()
        }
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

impl NavMesh {
    pub const RTTI: VariantID = RTTI_NavMesh;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NavMesh;
    pub const FORMTYPE: FormType = FormType::NavMesh;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }
}
