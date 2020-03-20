use super::Intersector;
use crate::raytracer::{intersect, Hit};
use crate::scene::Scene;
use crate::vecmath::Ray;

// no acceleration for intersections, just iterates through all the geometries' triangles.
pub struct NoAccelerationIntersector {}

impl Intersector for NoAccelerationIntersector {
    fn new(_scene: &Scene) -> Self {
        NoAccelerationIntersector {}
    }
    fn intersect_ray(&self, scene: &Scene, ray: &Ray) -> Option<Hit> {
        let mut closest_hit = None;

        for (geom_idx, geom) in scene.geometries.iter().enumerate() {
            let intersections: Vec<Option<intersect::HitInfo>> = geom
                .transformed_vertices
                .chunks(3)
                .map(|tri_vertices| {
                    intersect::intersect(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])
                })
                .collect();

            for (vtx_idx, intersection) in intersections.iter().enumerate() {
                match (&closest_hit, intersection) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(hit_info)) => {
                        closest_hit = Some(Hit::new(hit_info.clone(), geom_idx, vtx_idx * 3));
                    }
                    (Some(hit), Some(hit_info)) => {
                        if hit_info.t < hit.hit_info.t {
                            closest_hit = Some(Hit::new(hit_info.clone(), geom_idx, vtx_idx * 3));
                        }
                    }
                }
            }
        }
        closest_hit
    }
}
