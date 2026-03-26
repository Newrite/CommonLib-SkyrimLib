use crate::re::NiMatrix3;

/// C++ `RE::NiQuaternion`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NiQuaternion {
    pub w: f32, // 00
    pub x: f32, // 04
    pub y: f32, // 08
    pub z: f32, // 0C
}

const _: () = assert!(core::mem::size_of::<NiQuaternion>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiQuaternion, w) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiQuaternion, x) == 0x04);
const _: () = assert!(core::mem::offset_of!(NiQuaternion, y) == 0x08);
const _: () = assert!(core::mem::offset_of!(NiQuaternion, z) == 0x0C);

impl NiQuaternion {
    #[inline(always)]
    pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    #[inline(always)]
    pub const fn dot(self, other: Self) -> f32 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn correct(&mut self, to: Self) {
        if self.dot(to) < 0.0 {
            self.neg();
        }
    }

    #[inline(always)]
    pub fn neg(&mut self) {
        self.w = -self.w;
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    #[inline(always)]
    pub fn to_rotation(self) -> NiMatrix3 {
        let mut mat = NiMatrix3 {
            entry: [[0.0; 3]; 3],
        };
        self.write_rotation(&mut mat);
        mat
    }

    #[inline(always)]
    pub fn write_rotation(self, mat: &mut NiMatrix3) {
        let two_x = self.x * 2.0;
        let two_z = self.z * 2.0;
        let two_x2 = self.x * two_x;
        let two_xw = self.w * two_x;
        let two_y2 = self.y * self.y * 2.0;
        let two_yw = self.w * self.y * 2.0;
        let two_xy = self.x * self.y * 2.0;
        let two_xz = self.x * two_z;
        let two_zw = self.w * two_z;
        let v12 = self.z * two_z + two_x2;

        mat.entry[0][0] = 1.0 - (self.z * two_z + two_y2);
        mat.entry[1][0] = two_xy + two_zw;
        mat.entry[0][1] = two_xy - two_zw;
        mat.entry[2][0] = two_xz - two_yw;
        mat.entry[0][2] = two_xz + two_yw;
        mat.entry[1][2] = (self.y * two_z) - two_xw;
        mat.entry[1][1] = 1.0 - v12;
        mat.entry[2][1] = (self.y * two_z) + two_xw;
        mat.entry[2][2] = 1.0 - (two_y2 + two_x2);
    }
}
