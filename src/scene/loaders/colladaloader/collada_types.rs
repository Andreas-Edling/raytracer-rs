use crate::scene::{
    Light,
};

pub struct ColladaCamera {
    pub id: String,
    pub fov: f32,
    pub _aspect_ratio: f32,
    pub _scene_matrix: crate::vecmath::Matrix,
}

pub struct ColladaLight {
    pub id: String,
    pub light: Light,
}

pub struct ColladaEffect;

pub struct ColladaMaterial;

pub struct ColladaGeometry {
    pub vertices: Vec<f32>,
    pub triangles: Vec<u32>,
    pub id: String,
}

pub struct ColladaVisualScene {
    pub nodes: Vec<ColladaVisualSceneNode>,
}

pub struct ColladaVisualSceneNode {
    pub id: String,
    pub matrix: ColladaMatrix,
}

impl ColladaVisualSceneNode {
    pub fn new(id: String, matrix: ColladaMatrix) -> Self {
        ColladaVisualSceneNode {
            id,
            matrix,
        }
    }
}

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

        reflect_z * row_major  * swap_yx
    }
}



#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn collada_mat_to_vecmat_translation() {
        let cm = ColladaMatrix::from_slice(&[
            0.0, 0.0, 0.0, 10.0,
            0.0, 0.0, 0.0, 20.0,
            0.0, 0.0, 0.0, 30.0,
            0.0, 0.0, 0.0, 1.0,
        ]).unwrap();
        let m = cm.to_vecmath_matrix();
        let expected = crate::vecmath::Matrix::new(&[
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            10.0, 30.0, 20.0, 1.0,
        ]);
        assert_eq!(m, expected);
    }

    #[test]
    fn collada_mat_to_vecmat_z_vec() {
        let cm = ColladaMatrix::from_slice(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).unwrap();
        let m = cm.to_vecmath_matrix();
        
        let z_vec = Vec3::new(0.0,0.0,1.0);
        let expected = Vec3::new(0.0,-1.0,0.0);

        let actual = Vec3::from(m * crate::vecmath::Vec4::from_vec3(&z_vec));

        assert_eq!(actual, expected);
    }
}