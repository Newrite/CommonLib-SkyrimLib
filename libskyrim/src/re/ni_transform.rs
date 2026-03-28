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
    #[inline(always)]
    pub const fn identity() -> Self {
        Self {
            rotate: NiMatrix3 {
                entry: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            },
            translate: NiPoint3::new(0.0, 0.0, 0.0),
            scale: 1.0,
        }
    }

    #[inline(always)]
    fn transpose(matrix: &NiMatrix3) -> NiMatrix3 {
        NiMatrix3 {
            entry: [
                [matrix.entry[0][0], matrix.entry[1][0], matrix.entry[2][0]],
                [matrix.entry[0][1], matrix.entry[1][1], matrix.entry[2][1]],
                [matrix.entry[0][2], matrix.entry[1][2], matrix.entry[2][2]],
            ],
        }
    }

    #[inline(always)]
    fn mul_matrix(lhs: &NiMatrix3, rhs: &NiMatrix3) -> NiMatrix3 {
        let mut out = [[0.0; 3]; 3];
        let mut row = 0;
        while row < 3 {
            let mut col = 0;
            while col < 3 {
                out[row][col] = lhs.entry[row][0] * rhs.entry[0][col]
                    + lhs.entry[row][1] * rhs.entry[1][col]
                    + lhs.entry[row][2] * rhs.entry[2][col];
                col += 1;
            }
            row += 1;
        }
        NiMatrix3 { entry: out }
    }

    #[inline(always)]
    fn mul_point(matrix: &NiMatrix3, point: NiPoint3) -> NiPoint3 {
        NiPoint3::new(
            matrix.entry[0][0] * point.x
                + matrix.entry[0][1] * point.y
                + matrix.entry[0][2] * point.z,
            matrix.entry[1][0] * point.x
                + matrix.entry[1][1] * point.y
                + matrix.entry[1][2] * point.z,
            matrix.entry[2][0] * point.x
                + matrix.entry[2][1] * point.y
                + matrix.entry[2][2] * point.z,
        )
    }

    #[inline(always)]
    pub fn invert(&self) -> Self {
        let rotate = Self::transpose(&self.rotate);
        let scale = 1.0 / self.scale;
        let translate = Self::mul_point(&rotate, -self.translate) * scale;
        Self {
            rotate,
            translate,
            scale,
        }
    }

    #[inline(always)]
    pub fn not_equals(&self, rhs: &Self) -> bool {
        self != rhs
    }

    #[inline(always)]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        Self {
            scale: self.scale * rhs.scale,
            rotate: Self::mul_matrix(&self.rotate, &rhs.rotate),
            translate: self.translate + Self::mul_point(&self.rotate, rhs.translate) * self.scale,
        }
    }

    #[inline(always)]
    pub fn mul_point3(&self, point: NiPoint3) -> NiPoint3 {
        (Self::mul_point(&self.rotate, point) * self.scale) + self.translate
    }

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

impl Default for NiTransform {
    #[inline(always)]
    fn default() -> Self {
        Self::identity()
    }
}

unsafe extern "C" {
    fn atan2f(y: f32, x: f32) -> f32;
    fn sqrtf(x: f32) -> f32;
}
