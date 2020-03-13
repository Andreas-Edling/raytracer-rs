use super::Hit;
use crate::scene::Scene;
use crate::vecmath::Ray;

pub mod no_acceleration_intersector;
pub use no_acceleration_intersector::NoAccelerationIntersector;

pub mod oct_tree_intersector;
pub use oct_tree_intersector::OctTreeIntersector;

pub trait Intersector {
    fn new(scene: &Scene) -> Self;
    fn intersect_ray(&self, scene: &Scene, ray: &Ray) -> Option<Hit>;
}
