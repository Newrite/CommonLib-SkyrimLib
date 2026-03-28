/// C++ `RE::GMatrix2D`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GMatrix2D {
    pub data: [[f32; 3]; 2], // 00
}

const _: () = assert!(core::mem::size_of::<GMatrix2D>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GMatrix2D, data) == 0x0);

impl Default for GMatrix2D {
    #[inline(always)]
    fn default() -> Self {
        let mut matrix = Self {
            data: [[0.0; 3]; 2],
        };
        matrix.set_identity();
        matrix
    }
}

impl GMatrix2D {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn with_values(a_v0: f32, a_v1: f32, a_v2: f32, a_v3: f32, a_v4: f32, a_v5: f32) -> Self {
        let mut matrix = Self {
            data: [[0.0; 3]; 2],
        };
        matrix.set_matrix_values(a_v0, a_v1, a_v2, a_v3, a_v4, a_v5);
        matrix
    }

    #[inline(always)]
    pub fn set_matrix(&mut self, matrix: &Self) {
        self.data[0][0] = matrix.data[0][0];
        self.data[0][1] = matrix.data[0][1];
        self.data[0][2] = matrix.data[0][2];
        self.data[1][0] = matrix.data[1][0];
        self.data[1][1] = matrix.data[1][1];
        self.data[1][2] = matrix.data[1][2];
    }

    #[inline(always)]
    pub fn set_matrix_values(
        &mut self,
        a_v0: f32,
        a_v1: f32,
        a_v2: f32,
        a_v3: f32,
        a_v4: f32,
        a_v5: f32,
    ) {
        self.data[0][0] = a_v0;
        self.data[0][1] = a_v1;
        self.data[0][2] = a_v4;
        self.data[1][0] = a_v2;
        self.data[1][1] = a_v3;
        self.data[1][2] = a_v5;
    }

    #[inline(always)]
    pub fn set_identity(&mut self) {
        self.data[0][0] = 1.0;
        self.data[0][1] = 0.0;
        self.data[0][2] = 0.0;
        self.data[1][0] = 0.0;
        self.data[1][1] = 1.0;
        self.data[1][2] = 0.0;
    }
}
