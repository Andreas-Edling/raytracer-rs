use crate::scene::{Ray, Vertex};

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}
impl HitInfo {
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        Self { t, u, v }
    }
}

pub fn intersect(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<HitInfo> {
    moller_trumbore::intersect_late_out(ray, v0, v1, v2)
}

mod moller_trumbore {

    use super::HitInfo;
    use crate::scene::{Ray, Vertex};
    use crate::vecmath::{cross, dot};

    #[allow(dead_code)]
    pub fn intersect(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<HitInfo> {
        // Möller-Trumbore algo

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = cross(&ray.dir, &v0v2);
        let det = dot(&v0v1, &pvec);

        // ray and triangle are parallel if det is close to 0
        if det.abs() < std::f32::EPSILON {
            // switch to "if det < std::f32::EPSILON { return None };" for backface culling
            return None;
        }
        let inv_det = 1.0 / det;

        let tvec = ray.pos - v0;
        let u = dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = cross(&tvec, &v0v1);
        let v = dot(&ray.dir, &qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // u,v are coords in tri, return if needed
        let t = dot(&v0v2, &qvec) * inv_det;
        if t < 0.0 {
            return None;
        }
        Some(HitInfo::new(t, u, v))
    }

    #[allow(dead_code)]
    pub fn intersect_late_out(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<HitInfo> {
        // Möller-Trumbore algo

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = cross(&ray.dir, &v0v2);
        let det = dot(&v0v1, &pvec);
        // ray and triangle are parallel if det is close to 0
        if det.abs() < std::f32::EPSILON {
            // switch to "if det < std::f32::EPSILON { return None };" for backface culling
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.pos - v0;
        let u = dot(&tvec, &pvec) * inv_det;

        let qvec = cross(&tvec, &v0v1);
        let v = dot(&ray.dir, &qvec) * inv_det;

        // u,v are coords in tri, return if needed
        let t = dot(&v0v2, &qvec) * inv_det;

        // dont merge re-order or break apart these if-clauses - it has a major performance impact!
        if u < 0.0 || u > 1.0 {
            return None;
        }
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        if t < 0.0 {
            return None;
        }

        Some(HitInfo::new(t, u, v))
    }

    #[allow(dead_code)]
    pub fn intersect_later_out(
        ray: &Ray,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
    ) -> Option<HitInfo> {
        // Möller-Trumbore algo

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = cross(&ray.dir, &v0v2);
        let det = dot(&v0v1, &pvec);
        let tvec = ray.pos - v0;
        let qvec = cross(&tvec, &v0v1);

        let u = dot(&tvec, &pvec);
        let v = dot(&ray.dir, &qvec);
        let t = dot(&v0v2, &qvec);

        // ray and triangle are parallel if det is close to 0
        if det.abs() < std::f32::EPSILON {
            // switch to "if det < std::f32::EPSILON { return None };" for backface culling
            return None;
        }

        let inv_det = 1.0 / det;
        let u = u * inv_det;
        let v = v * inv_det;
        // u,v are coords in tri, return if needed
        let t = t * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        if t < 0.0 {
            return None;
        }

        Some(HitInfo::new(t, u, v))
    }
}
