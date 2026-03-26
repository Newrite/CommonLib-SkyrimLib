/// C++ `RE::NiMatrix44`
///
/// TODO: `NiCamera.h` only forward-declares `NiMatrix44`, but its VR runtime
/// data embeds `BSTArray<NiMatrix44>`, which needs a sized Rust element type.
/// This minimal stand-in models the conventional 4x4 float matrix storage used
/// by the surrounding camera math and should be replaced once a same-name
/// CommonLib definition is sourced into the repo.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NiMatrix44 {
    pub entry: [[f32; 4]; 4], // 00
}

const _: () = assert!(core::mem::size_of::<NiMatrix44>() == 0x40);
