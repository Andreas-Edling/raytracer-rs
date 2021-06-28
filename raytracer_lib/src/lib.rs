mod raytracer;
mod scene;
mod vecmath;

pub mod stats;
pub use raytracer::RayTracer;
pub use raytracer::accel_intersect::oct_tree_intersector::DEFAULT_TRIANGLES_PER_LEAF;


#[allow(unused_imports)]
use scene::loaders::{colladaloader::ColladaLoader, SceneLoader};

use scene::Scene;

pub fn create_raytracer(collada_doc: &str, triangles_per_leaf: usize, width: usize, height: usize) -> Result<RayTracer, String> {
    let scene = ColladaLoader::from_str(collada_doc, None, width, height)
        .map_err(|e| e.to_string())?;

    build_raytracer(scene, triangles_per_leaf, width, height)
}

pub fn create_raytracer_from_file(collada_filename: String, triangles_per_leaf: usize, width: usize, height: usize) -> Result<RayTracer, String> {
    let scene = ColladaLoader::from_file(collada_filename, width, height)
        .map_err(|e| e.to_string())?;

    build_raytracer(scene, triangles_per_leaf, width, height)
}

fn build_raytracer(scene: Scene, triangles_per_leaf: usize, width: usize, height: usize) -> Result<RayTracer, String> {
    let octtree = raytracer::accel_intersect::OctTreeIntersector::with_triangles_per_leaf(
        &scene,
        triangles_per_leaf,
    );
    
    Ok(
        RayTracer::new_with_intersector(
            width,
            height,
            scene.cameras[0].clone(),
            octtree,
            scene,
        )
    )
}