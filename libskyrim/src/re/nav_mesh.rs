use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_NavMesh;
use crate::offsets::offsets_vtable::VTABLE_NavMesh;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{BSNavmesh, Form, FormCastable, FormType, TESChildCell, TESForm};
use crate::relocation::{RttiType, VariantID};

bitflags! {
    /// C++ `RE::NavMesh::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NavMeshRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
        const COMPRESSED = 1 << 18;
        const AUTO_GEN = 1 << 26;
        const NAV_MESH_GEN_CELL = 1u32 << 31;
    }
}

/// C++ `RE::NavMesh`
#[repr(C)]
pub struct NavMesh {
    pub base: TESForm,            // 00
    pub child_cell: TESChildCell, // 20
    pub navmesh: BSNavmesh,       // 28
}

const _: () = assert!(core::mem::size_of::<NavMesh>() == 0x140);
const _: () = assert!(core::mem::offset_of!(NavMesh, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NavMesh, child_cell) == 0x20);
const _: () = assert!(core::mem::offset_of!(NavMesh, navmesh) == 0x28);

impl RttiType for NavMesh {
    const RTTI: VariantID = RTTI_NavMesh;
}

impl FormCastable for NavMesh {
    const TARGET_FORM_TYPE: FormType = FormType::NavMesh;
}

inherit!(NavMesh : TESForm, base);

impl AsRef<TESChildCell> for NavMesh {
    #[inline(always)]
    fn as_ref(&self) -> &TESChildCell {
        &self.child_cell
    }
}

impl AsMut<TESChildCell> for NavMesh {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut TESChildCell {
        &mut self.child_cell
    }
}

impl AsRef<BSNavmesh> for NavMesh {
    #[inline(always)]
    fn as_ref(&self) -> &BSNavmesh {
        &self.navmesh
    }
}

impl AsMut<BSNavmesh> for NavMesh {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BSNavmesh {
        &mut self.navmesh
    }
}

impl BSTSmartPointerIntrusiveRefCountable for NavMesh {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.navmesh.bst_inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.navmesh.bst_dec_ref()
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

    crate::virtual_method! {
        pub const VFUNC_SAVE: usize = 0x3B;
        pub fn save(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SAVES_BEFORE_FORM: usize = 0x3C;
        pub fn saves_before_form(&mut self, form: *mut Form) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SAVES_BEFORE_TES_FORM: usize = 0x3D;
        pub fn saves_before_tes_form(&mut self, form: *mut TESForm) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS_BEFORE_SAVE: usize = 0x3E;
        pub fn process_before_save(&mut self) -> bool
    }
}
