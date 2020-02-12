mod accel_intersect;
mod film;
mod intersect;
mod sample_generator;

use crate::scene::{camera::Camera, color::RGB, Ray, Scene};
use crate::vecmath::{cross, dot, Vec3};

use accel_intersect::*;
use film::Film;
use sample_generator::SampleGenerator;
use timing::BenchMark;

pub struct Hit {
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

pub struct RayTracer<Accel = OctTreeAccelerationIntersector>
where
    Accel: Intersector,
{
    width: usize,
    height: usize,
    pub camera: Camera,

    sample_generator: sample_generator::SampleGenerator,
    pub film: Film,
    accel: Accel,
}

impl RayTracer {
    pub fn new(width: usize, height: usize, camera: Camera, scene: &Scene) -> Self {
        RayTracer {
            width,
            height,
            camera,
            sample_generator: SampleGenerator::new(),
            film: Film::new(width * height),
            accel: Intersector::new(scene),
        }
    }

    pub fn trace_frame_additive(&mut self, scene: &Scene, _timer: &mut BenchMark) {
        const RECURSIONS: u8 = 2;
        const SUB_SPREAD: u32 = 1;

        let mut rng = rand::thread_rng();

        for (i, pixel_and_sample_count) in self.film.pixels_and_sample_counts.iter_mut().enumerate()
        {
            let ray = self
                .camera
                .get_ray(i % self.width, i / self.height, &mut rng);

            let hit = self.accel.intersect_ray(&scene, &ray);
            let color = match hit {
                None => RGB::black(),
                Some(ref hit) => compute_radiance(
                    &self.accel,
                    &scene,
                    &ray,
                    hit,
                    &mut self.sample_generator,
                    RECURSIONS,
                    SUB_SPREAD,
                ),
            };
            pixel_and_sample_count.add_sample(color);
        }
    }
}

fn compute_radiance<Accel>(
    accel: &Accel,
    scene: &Scene,
    ray: &Ray,
    hit: &Hit,
    sample_generator: &mut SampleGenerator,
    recursions: u8,
    spread: u32,
) -> RGB
where
    Accel: Intersector,
{
    let normal = calc_normal(scene, &hit);
    let radiance = shade(scene, ray, hit, &normal);
    if recursions < 1 {
        return radiance;
    }

    let num_sub_rays = spread * recursions as u32;

    let sub_radiance = (0..num_sub_rays)
        .map(|_| {
            let sub_ray = randomize_reflection_ray(sample_generator, hit, ray, &normal);

            let sub_hit = accel.intersect_ray(&scene, &sub_ray);

            match sub_hit {
                Some(sub_hit) => compute_radiance(
                    accel,
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
