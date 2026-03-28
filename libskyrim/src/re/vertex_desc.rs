/// C++ `RE::BSGraphics::Vertex::Attribute`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSGraphicsVertexAttribute {
    Position = 0x0,
    TexCoord0 = 0x1,
    TexCoord1 = 0x2,
    Normal = 0x3,
    Binormal = 0x4,
    Color = 0x5,
    Skinning = 0x6,
    LandData = 0x7,
    EyeData = 0x8,
    InstanceData = 0x9,
}

/// C++ `RE::BSGraphics::Vertex::Flags`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSGraphicsVertexFlags {
    Vertex = 1 << 0,
    Uv = 1 << 1,
    Uv2 = 1 << 2,
    Normal = 1 << 3,
    Tangent = 1 << 4,
    Colors = 1 << 5,
    Skinned = 1 << 6,
    LandData = 1 << 7,
    EyeData = 1 << 8,
    InstanceData = 1 << 9,
    FullPrec = 0x400,
}

core_util::impl_enumset_type!(BSGraphicsVertexFlags => u16);

pub struct BSGraphicsVertexMasks;

impl BSGraphicsVertexMasks {
    pub const DESC_MASK_VERT: u64 = 0xFFFF_FFFF_FFFF_FFF0;
    pub const DESC_MASK_UVS: u64 = 0xFFFF_FFFF_FFFF_FF0F;
    pub const DESC_MASK_NBT: u64 = 0xFFFF_FFFF_FFFF_F0FF;
    pub const DESC_MASK_SKCOL: u64 = 0xFFFF_FFFF_FFFF_0FFF;
    pub const DESC_MASK_DATA: u64 = 0xFFFF_FFFF_FFF0_FFFF;
    pub const DESC_MASK_OFFSET: u64 = 0xFFFF_FF00_0000_0000;
    pub const DESC_MASK_FLAGS: u64 = !Self::DESC_MASK_OFFSET;
}

/// C++ `RE::BSGraphics::VertexDesc`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BSGraphicsVertexDesc {
    pub desc: u64, // 00
}

const _: () = assert!(core::mem::size_of::<BSGraphicsVertexDesc>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSGraphicsVertexDesc, desc) == 0x00);

impl BSGraphicsVertexDesc {
    #[inline(always)]
    pub const fn has_flag(self, flag: BSGraphicsVertexFlags) -> bool {
        ((self.desc >> 44) & flag as u64) != 0
    }

    #[inline(always)]
    pub fn set_flag(&mut self, flag: BSGraphicsVertexFlags) {
        self.desc |= (flag as u64) << 44;
    }

    #[inline(always)]
    pub fn clear_flag(&mut self, flag: BSGraphicsVertexFlags) {
        self.desc &= !((flag as u64) << 44);
    }

    #[inline(always)]
    pub const fn get_attribute_offset(self, attribute: BSGraphicsVertexAttribute) -> u32 {
        ((self.desc >> (4 * attribute as u8 + 2)) & 0x3C) as u32
    }

    #[inline(always)]
    pub fn set_attribute_offset(&mut self, attribute: BSGraphicsVertexAttribute, offset: u32) {
        if !matches!(attribute, BSGraphicsVertexAttribute::Position) {
            let shift = 4 * attribute as u8 + 2;
            let lhs = (offset as u64) << shift;
            let rhs = self.desc & !(15u64 << (4 * attribute as u8 + 4));
            self.desc = lhs | rhs;
        }
    }

    #[inline(always)]
    pub fn clear_attribute_offsets(&mut self) {
        self.desc &= BSGraphicsVertexMasks::DESC_MASK_OFFSET;
    }

    #[inline(always)]
    pub const fn get_flags(self) -> u16 {
        ((self.desc & BSGraphicsVertexMasks::DESC_MASK_OFFSET) >> 44) as u16
    }

    #[inline(always)]
    pub fn set_flags_bits(&mut self, flags: u16) {
        self.desc |= ((flags as u64) << 44) | (self.desc & BSGraphicsVertexMasks::DESC_MASK_FLAGS);
    }

    #[inline(always)]
    pub fn get_size(self) -> u32 {
        let flags = self.get_flags();
        let has = |flag: BSGraphicsVertexFlags| (flags & flag as u16) != 0;

        let mut vertex_size = 0;
        if has(BSGraphicsVertexFlags::Vertex) {
            vertex_size += core::mem::size_of::<f32>() as u32 * 4;
        }
        if has(BSGraphicsVertexFlags::Uv) {
            vertex_size += core::mem::size_of::<u16>() as u32 * 2;
        }
        if has(BSGraphicsVertexFlags::Uv2) {
            vertex_size += core::mem::size_of::<u16>() as u32 * 2;
        }
        if has(BSGraphicsVertexFlags::Normal) {
            vertex_size += core::mem::size_of::<u16>() as u32 * 2;
            if has(BSGraphicsVertexFlags::Tangent) {
                vertex_size += core::mem::size_of::<u16>() as u32 * 2;
            }
        }
        if has(BSGraphicsVertexFlags::Colors) {
            vertex_size += core::mem::size_of::<u8>() as u32 * 4;
        }
        if has(BSGraphicsVertexFlags::Skinned) {
            vertex_size += core::mem::size_of::<u16>() as u32 * 4;
            vertex_size += core::mem::size_of::<u8>() as u32 * 4;
        }
        if has(BSGraphicsVertexFlags::EyeData) {
            vertex_size += core::mem::size_of::<f32>() as u32;
        }

        vertex_size
    }
}
