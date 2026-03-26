use crate::re::NiPoint3;

/// Partial translation of `RE::BSNavmesh.h` used by `NavMesh` consumers.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSNavmeshVertex {
    pub location: NiPoint3, // 00
}

const _: () = assert!(core::mem::size_of::<BSNavmeshVertex>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSNavmeshVertex, location) == 0x00);
