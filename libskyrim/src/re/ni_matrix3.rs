/// C++ `RE::NiMatrix3`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiMatrix3 {
    pub entry: [[f32; 3]; 3], // 00
}

const _: () = assert!(core::mem::size_of::<NiMatrix3>() == 0x24);
const _: () = assert!(core::mem::offset_of!(NiMatrix3, entry) == 0x00);

impl NiMatrix3 {
    pub fn to_euler_angles_xyz(&self, angle: &mut crate::re::NiPoint3) -> bool {
        self.to_euler_angles_xyz_values(&mut angle.x, &mut angle.y, &mut angle.z)
    }

    pub fn to_euler_angles_xyz_values(
        &self,
        x_angle: &mut f32,
        y_angle: &mut f32,
        z_angle: &mut f32,
    ) -> bool {
        *y_angle = -unsafe { asinf(self.entry[0][2]) };
        if *y_angle < core::f32::consts::FRAC_PI_2 {
            if *y_angle > -core::f32::consts::FRAC_PI_2 {
                *x_angle = -unsafe { atan2f(-self.entry[1][2], self.entry[2][2]) };
                *z_angle = -unsafe { atan2f(-self.entry[0][1], self.entry[0][0]) };
                true
            } else {
                let rm_y = unsafe { atan2f(self.entry[1][0], self.entry[1][1]) };
                *z_angle = 0.0;
                *x_angle = rm_y - *z_angle;
                false
            }
        } else {
            let rp_y = unsafe { atan2f(self.entry[1][0], self.entry[1][1]) };
            *z_angle = 0.0;
            *x_angle = *z_angle - rp_y;
            false
        }
    }
}

unsafe extern "C" {
    fn asinf(x: f32) -> f32;
    fn atan2f(y: f32, x: f32) -> f32;
}
