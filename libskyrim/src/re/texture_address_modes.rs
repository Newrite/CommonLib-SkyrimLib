/// C++ `RE::BSGraphics::TextureAddressMode`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSGraphicsTextureAddressMode {
    ClampSClampT = 0,
    ClampSWrapT = 1,
    WrapSClampT = 2,
    WrapSWrapT = 3,
}

core_util::impl_enumset_type!(BSGraphicsTextureAddressMode => u8);
