mod raytracer;
mod scene;
mod vecmath;

pub mod stats;
pub use raytracer::RayTracer;
pub use raytracer::accel_intersect::oct_tree_intersector::DEFAULT_TRIANGLES_PER_LEAF;


#[allow(unused_imports)]
use scene::loaders::{colladaloader::ColladaLoader, SceneLoader};


pub fn create_raytracer(collada_filename: String, triangles_per_leaf: usize, width: usize, height: usize) -> Result<RayTracer, String> {
    let scene = ColladaLoader::from_file(collada_filename, width, height)
        .map_err(|e| e.to_string())?;

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

