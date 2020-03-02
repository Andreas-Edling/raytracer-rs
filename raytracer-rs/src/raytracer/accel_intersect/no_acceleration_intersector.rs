use crate::raytracer::{intersect, Hit};
use crate::scene::Scene;
use crate::vecmath::Ray;
use super::Intersector;


// no acceleration for intersections, just iterates through all the geometries' triangles.
pub struct NoAccelerationIntersector {}

impl Intersector for NoAccelerationIntersector {
    fn new(_scene: &Scene) -> Self {
        NoAccelerationIntersector {}
    }
    fn intersect_ray(&self, scene: &Scene, ray: &Ray) -> Option<Hit> {
        let mut closest_hit = None;

        for (geom_idx, geom) in scene.geometries.iter().enumerate() {
            let intersect_distances: Vec<Option<f32>> = geom
                .transformed_vertices
                .chunks(3)
                .map(|tri_vertices| {
                    intersect::intersect(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])
                })
                .collect();

            for (vtx_idx, intersect_distance) in intersect_distances.iter().enumerate() {
                match (&closest_hit, intersect_distance) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(dist)) => {
                        closest_hit = Some(Hit::new(*dist, geom_idx, vtx_idx * 3));
                    }
                    (Some(hit), Some(dist)) => {
                        if *dist < hit.distance {
                            closest_hit = Some(Hit::new(*dist, geom_idx, vtx_idx * 3));
                        }
                    }
                }
            }
        }
        closest_hit
    }
}
