#[derive(Debug, Clone, PartialEq)]
pub struct ColladaMatrix {
    //Collada uses right handed, with Z up, and column major order (positions in 4th column).
    elems: [f32;16],
}

impl ColladaMatrix {
    pub fn from_slice(elems: &[f32]) -> Option<Self> {
        if elems.len()<16 {
            return None;
        }

        let mut array = [0.0; 16];
        let elems = &elems[..16];
        array.copy_from_slice(elems); 
        Some(ColladaMatrix{ elems: array })
    }

    pub fn to_vecmath_matrix(&self) -> crate::vecmath::Matrix {
        //transforms to_left_handed_y_up_row_major

        let row_major = crate::vecmath::Matrix::new(&self.elems).transpose();

        let swap_yx = crate::vecmath::Matrix::new(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let reflect_z = crate::vecmath::Matrix::new(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let result = reflect_z * row_major  * swap_yx;
        result
    }
}
