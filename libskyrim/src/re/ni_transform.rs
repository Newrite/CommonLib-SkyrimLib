use crate::re::{NiMatrix3, NiPoint3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiTransform {
    pub rotate: NiMatrix3,   // 00
    pub translate: NiPoint3, // 24
    pub scale: f32,          // 30
}

unsafe impl bytemuck::Zeroable for NiTransform {}

const _: () = assert!(core::mem::size_of::<NiTransform>() == 0x34);
const _: () = assert!(core::mem::offset_of!(NiTransform, rotate) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiTransform, translate) == 0x24);
const _: () = assert!(core::mem::offset_of!(NiTransform, scale) == 0x30);

impl NiTransform {
    pub fn get_heading_angle(&self, target_pos: &NiPoint3, abs: bool) -> f32 {
        let sy = unsafe {
            sqrtf(
                self.rotate.entry[0][0] * self.rotate.entry[0][0]
                    + self.rotate.entry[0][1] * self.rotate.entry[0][1],
            )
        };
        let angle_z = if sy < 1e-6 {
            0.0
        } else {
            unsafe { atan2f(self.rotate.entry[0][1], self.rotate.entry[0][0]) }
        };

        let theta = unsafe {
            atan2f(
                target_pos.x - self.translate.x,
                target_pos.y - self.translate.y,
            )
        };
        let mut heading = (theta - angle_z).to_degrees();

        if heading < -180.0 {
            heading += 360.0;
        }

        if heading > 180.0 {
            heading -= 360.0;
        }

        if abs { heading.abs() } else { heading }
    }
}

unsafe extern "C" {
    fn atan2f(y: f32, x: f32) -> f32;
    fn sqrtf(x: f32) -> f32;
}
