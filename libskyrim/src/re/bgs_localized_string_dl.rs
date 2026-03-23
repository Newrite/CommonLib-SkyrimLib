#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, bytemuck::Zeroable)]
pub struct BGSLocalizedStringDL {
    pub id: u32,
}

const _: () = assert!(core::mem::size_of::<BGSLocalizedStringDL>() == 0x4);
