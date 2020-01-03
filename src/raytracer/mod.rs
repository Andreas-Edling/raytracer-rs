
use rand::Rng;

use crate::vecmath::{cross, dot, Vec3};

use crate::scene::{
    Scene,
    Vertex,
    color::RGB,
    camera::Camera,
    Ray,
};


#[allow(dead_code)]
fn intersect(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<f32> {
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

    let tvec = &ray.pos - v0;
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
    if t<0.0 {
        return None;
    }
    Some(t)
}

#[allow(dead_code)]
fn intersect_late_out(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<f32> {
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

    let tvec = &ray.pos - v0;
    let u = dot(&tvec, &pvec) * inv_det;

    let qvec = cross(&tvec, &v0v1);
    let v = dot(&ray.dir, &qvec) * inv_det;

    // u,v are coords in tri, return if needed
    let t = dot(&v0v2, &qvec) * inv_det;


    // dont merge re-order or break apart these if-clauses - it has a major performance impact!
    if u < 0.0 || u > 1.0 { return None; }
    if v < 0.0 || u + v > 1.0 { return None; }
    if t<0.0 { return None; }

    Some(t)
}

#[allow(dead_code)]
fn intersect_later_out(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Option<f32> {
    // Möller-Trumbore algo

    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let pvec = cross(&ray.dir, &v0v2);
    let det = dot(&v0v1, &pvec);
    let tvec = &ray.pos - v0;
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
    if t<0.0 {
        return None;
    }

    Some(t)
}

struct Hit {
    distance: f32,
    geometry_index: usize,
    vertex_index: usize,
}
impl Hit {
    fn new(distance: f32, geometry_index: usize, vertex_index: usize) -> Self {
        Hit{ distance, geometry_index, vertex_index}
    }
}

pub struct RayTracer {
    width: usize,
    height: usize,
    pub camera: Camera,
}


impl RayTracer {
    pub fn new(width: usize, height: usize, camera: Camera) -> Self {
        RayTracer {
            width,
            height,
            camera,
        }
    }

    #[rustfmt::skip]
    pub fn trace_frame(&mut self, scene: &Scene) -> Vec<RGB> {
        let rays = {self.camera.get_rays()};
        let mut frame = Vec::with_capacity(self.width*self.height);

        for ray in rays {
            let hit = Self::trace_ray(scene, ray);

            let color = match hit {
                Some(ref hit) => {
                    let normal = Self::calc_normal(scene, &hit);
                    let radiance = Self::shade(scene, ray, hit, &normal);
                    
                    const SUB_RAYS: u32 = 100;
                    let sub_radiance = (0..SUB_RAYS).map(|_|{
                        let sub_ray = Self::randomize_reflection_ray(hit, ray, &normal);
                        match Self::trace_ray(scene, &sub_ray) {
                            Some(sub_hit) => {
                                let sub_normal = Self::calc_normal(scene, &sub_hit);
                                let sub_radiance = Self::shade(scene, &sub_ray, &sub_hit, &sub_normal);
                                sub_radiance 
                            },
                            None => RGB::black(),
                        }
                    }).fold(RGB::black(), |sum, x| sum + x) * (1.0f32 / SUB_RAYS as f32);
                    radiance + sub_radiance 
                },
                None => RGB::black()
            };
            frame.push(color);
        }

        frame
    }

    fn randomize_reflection_ray(hit: &Hit, ray: &Ray, normal: &Vec3) -> Ray {
        let mut rng = rand::thread_rng();

        let hit_point = &ray.pos + hit.distance * &ray.dir;

        let dir = loop {
            let dir = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );
            if dot(&dir, &dir) < 1.0 && dot(&dir, normal) > 0.0{
                break dir;
            }
        };

        let hit_point = hit_point + 0.00001*&dir;
        Ray::new(hit_point, dir)
    }


    fn trace_ray(scene: &Scene, ray: &Ray) -> Option<Hit> {
        let mut closest_hit = None;

        for (geom_idx, geom) in scene.geometries.iter().enumerate() {

            let intersect_distances: Vec<Option<f32>> = geom.transformed_vertices.chunks(3)
                .map( |tri_vertices| intersect_late_out(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]))
                .collect();

            for (vtx_idx, intersect_distance) in intersect_distances.iter().enumerate() {
                match (&closest_hit, intersect_distance) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(dist)) => {
                        closest_hit = Some(Hit::new(*dist, geom_idx, vtx_idx*3));
                    },
                    (Some(hit), Some(dist)) => {
                        if  *dist < hit.distance {
                            closest_hit = Some(Hit::new(*dist, geom_idx, vtx_idx*3));
                        }
                    },
                }
            }
        }
        closest_hit
    }


    fn calc_normal(scene: &Scene, hit: &Hit) -> Vec3 {
        let geom_vertices = &scene.geometries[hit.geometry_index].vertices;
        let normal = cross( 
            &(&geom_vertices[hit.vertex_index+1] - &geom_vertices[hit.vertex_index]), 
            &(&geom_vertices[hit.vertex_index+2] - &geom_vertices[hit.vertex_index]));
        let normal = normal.normalized();
        normal

    }

    fn shade(scene: &Scene, ray: &Ray, hit: &Hit, normal: &Vec3) -> RGB {
        let mut accum_color = RGB::black();
        let hit_point = &ray.pos + hit.distance * &ray.dir;

        for light in &scene.lights {
            let ray_to_light = Ray::new(hit_point.clone(), &light.pos - &hit_point);
            let dot_light_normal = dot(&normal, &ray_to_light.dir.normalized());
        
            if dot_light_normal < 0.0 {
                continue;  // triangle is facing away from light
            }

            //is light blocked by geometry?
            let mut blocked = false;
            for geom in &scene.geometries {
                for tri_vertices in geom.vertices.chunks(3) {
                    if let Some(t) = intersect_later_out(&ray_to_light, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]) {
                        if t > 0.0001 && t < 1.0 {
                            blocked = true;
                            break;
                        }
                    }
                }
            }

            if !blocked {
                //lambertian / diffuse
                accum_color += dot_light_normal * &light.color * &scene.geometries[hit.geometry_index].material.diffuse; 

                // phong
                // {
                //     const SPECULAR: f32 = 0.5;
                //     const DIFFUSE: f32 = 0.5;
                //     const SHININESS: f32 = 32.0;
                //     let view_ray = -ray.dir.normalized(); 
                //     let reflected_light = 2.0*dot_light_normal*normal - ray_to_light.dir.normalized();
                //     accum_color += (DIFFUSE*dot_light_normal + SPECULAR*dot(&view_ray, &reflected_light).powf(SHININESS)) * light.color;
                // }
            }
        }
        accum_color
    }
}
