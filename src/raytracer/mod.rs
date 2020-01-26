mod film;
mod intersect;
mod sample_generator;

use crate::vecmath::{cross, dot, Vec3};

use crate::scene::{camera::Camera, color::RGB, Ray, Scene};

use film::Film;
use sample_generator::SampleGenerator;
use timing::BenchMark;

struct Hit {
    distance: f32,
    geometry_index: usize,
    vertex_index: usize,
}
impl Hit {
    fn new(distance: f32, geometry_index: usize, vertex_index: usize) -> Self {
        Hit {
            distance,
            geometry_index,
            vertex_index,
        }
    }
}

pub struct RayTracer {
    width: usize,
    height: usize,
    pub camera: Camera,

    sample_generator: sample_generator::SampleGenerator,
    pub film: Film,
}

impl RayTracer {
    pub fn new(width: usize, height: usize, camera: Camera) -> Self {
        RayTracer {
            width,
            height,
            camera,
            sample_generator: SampleGenerator::new(),
            film: Film::new(width * height),
        }
    }

    #[allow(dead_code)]
    pub fn trace_frame(&mut self, scene: &Scene, _timer: &mut BenchMark) -> Vec<RGB> {
        let rays = self.camera.get_jittered_rays();
        let mut frame = Vec::with_capacity(self.width * self.height);

        const RECURSIONS: u8 = 1;
        const SUB_SPREAD: u32 = 1;

        for ray in rays {
            let hit = intersect_ray(scene, ray);
            let color = match hit {
                None => RGB::black(),
                Some(ref hit) => compute_radiance(
                    scene,
                    ray,
                    hit,
                    &mut self.sample_generator,
                    RECURSIONS,
                    SUB_SPREAD,
                ),
            };
            frame.push(color);
        }

        frame
    }

    pub fn trace_frame_additive(&mut self, scene: &Scene, _timer: &mut BenchMark) {
        let rays = self.camera.get_jittered_rays();

        const RECURSIONS: u8 = 2;
        const SUB_SPREAD: u32 = 1;

        for (ray, (film_samples, film_pixel)) in rays.iter().zip(self.film.iter_mut()) {
            let hit = intersect_ray(scene, ray);
            let color = match hit {
                None => RGB::black(),
                Some(ref hit) => compute_radiance(
                    scene,
                    ray,
                    hit,
                    &mut self.sample_generator,
                    RECURSIONS,
                    SUB_SPREAD,
                ),
            };
            *film_samples += 1;
            *film_pixel += color;
        }
    }
}

fn compute_radiance(
    scene: &Scene,
    ray: &Ray,
    hit: &Hit,
    sample_generator: &mut SampleGenerator,
    recursions: u8,
    spread: u32,
) -> RGB {
    let normal = calc_normal(scene, &hit);
    let radiance = shade(scene, ray, hit, &normal);
    if recursions < 1 {
        return radiance;
    }

    let num_sub_rays = spread * recursions as u32;

    let sub_radiance = (0..num_sub_rays)
        .map(|_| {
            let sub_ray = randomize_reflection_ray(sample_generator, hit, ray, &normal);

            let sub_hit = intersect_ray(scene, &sub_ray);

            match sub_hit {
                Some(sub_hit) => compute_radiance(
                    scene,
                    &sub_ray,
                    &sub_hit,
                    sample_generator,
                    recursions - 1,
                    spread,
                ),
                None => RGB::black(),
            }
        })
        .fold(RGB::black(), |sum, x| sum + x)
        * (1.0f32 / num_sub_rays as f32);
    radiance + sub_radiance
}

fn randomize_reflection_ray(
    sample_generator: &mut SampleGenerator,
    hit: &Hit,
    ray: &Ray,
    normal: &Vec3,
) -> Ray {
    // get random direction on hemisphere
    let mut random_dir = sample_generator.normalized_vec_pseudo();
    while dot(&random_dir, normal) <= 0.0 {
        random_dir = sample_generator.normalized_vec_lookup();
    }

    // calc pos and offset slightly
    let hit_point = &ray.pos + hit.distance * &ray.dir;
    let hit_point = hit_point + 0.00001 * &random_dir;

    Ray::new(hit_point, random_dir)
}

fn intersect_ray(scene: &Scene, ray: &Ray) -> Option<Hit> {
    let mut closest_hit = None;

    for (geom_idx, geom) in scene.geometries.iter().enumerate() {
        let intersect_distances: Vec<Option<f32>> = geom
            .transformed_vertices
            .chunks(3)
            .map(|tri_vertices| {
                intersect::intersect_late_out(
                    ray,
                    &tri_vertices[0],
                    &tri_vertices[1],
                    &tri_vertices[2],
                )
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

fn calc_normal(scene: &Scene, hit: &Hit) -> Vec3 {
    let geom_vertices = &scene.geometries[hit.geometry_index].vertices;
    let normal = cross(
        &(&geom_vertices[hit.vertex_index + 1] - &geom_vertices[hit.vertex_index]),
        &(&geom_vertices[hit.vertex_index + 2] - &geom_vertices[hit.vertex_index]),
    );
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
            continue; // triangle is facing away from light
        }

        //is light blocked by geometry?
        let mut blocked = false;
        for geom in &scene.geometries {
            for tri_vertices in geom.vertices.chunks(3) {
                if let Some(t) = intersect::intersect_later_out(
                    &ray_to_light,
                    &tri_vertices[0],
                    &tri_vertices[1],
                    &tri_vertices[2],
                ) {
                    if t > 0.0001 && t < 1.0 {
                        blocked = true;
                        break;
                    }
                }
            }
        }

        if !blocked {
            //lambertian / diffuse
            accum_color += dot_light_normal
                * &light.color
                * &scene.geometries[hit.geometry_index].material.diffuse;

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
