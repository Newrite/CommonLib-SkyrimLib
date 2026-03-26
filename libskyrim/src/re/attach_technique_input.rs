use crate::offsets::offsets_rtti::RTTI_BSAttachTechniques__AttachTechniqueInput;
use crate::offsets::offsets_vtable::VTABLE_BSAttachTechniques__AttachTechniqueInput;
use crate::re::{NiNode, NiPointer};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSAttachTechniques::AttachTechniqueInput`
#[repr(C)]
pub struct AttachTechniqueInput {
    pub vtable: *const usize,               // 00
    pub current_3d_root: NiPointer<NiNode>, // 08
    pub attached_art: NiPointer<NiNode>,    // 10
    pub attach_point: u32,                  // 18
    pub pad1c: u32,                         // 1C
}

const _: () = assert!(core::mem::size_of::<AttachTechniqueInput>() == 0x20);
const _: () = assert!(core::mem::offset_of!(AttachTechniqueInput, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(AttachTechniqueInput, current_3d_root) == 0x08);
const _: () = assert!(core::mem::offset_of!(AttachTechniqueInput, attached_art) == 0x10);
const _: () = assert!(core::mem::offset_of!(AttachTechniqueInput, attach_point) == 0x18);

impl RttiType for AttachTechniqueInput {
    const RTTI: VariantID = RTTI_BSAttachTechniques__AttachTechniqueInput;
}

impl AsRef<AttachTechniqueInput> for AttachTechniqueInput {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<AttachTechniqueInput> for AttachTechniqueInput {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AttachTechniqueInput {
    pub const RTTI: VariantID = RTTI_BSAttachTechniques__AttachTechniqueInput;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSAttachTechniques__AttachTechniqueInput;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR: usize = 0x01;
        pub fn clear()
    }
}

pub trait AttachTechniqueInputExt {
    fn dtor(&mut self);
    fn clear(&mut self);
}

impl<T: AsRef<AttachTechniqueInput> + AsMut<AttachTechniqueInput>> AttachTechniqueInputExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        AttachTechniqueInput::dtor(self.as_mut())
    }

    #[inline(always)]
    fn clear(&mut self) {
        AttachTechniqueInput::clear(self.as_mut())
    }
}
