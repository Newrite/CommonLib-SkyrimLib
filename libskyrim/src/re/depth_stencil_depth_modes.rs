/// C++ `RE::BSGraphics::DepthStencilDepthMode`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSGraphicsDepthStencilDepthMode {
    Disabled = 0,
    Test = 1,
    Write = 2,
    TestWrite = 3,
    TestEqual = 4,
    TestGreaterEqual = 5,
    TestGreater = 6,
}
