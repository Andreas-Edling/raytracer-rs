pub mod accel_intersect;
mod film;
mod intersect;
mod sample_generator;

use crate::scene::{camera::Camera, color::RGB, color::Diffuse, Ray, Scene};
use crate::vecmath::{cross, dot, Vec3};

use intersect::HitInfo;
use accel_intersect::*;
use film::Film;
use sample_generator::SampleGenerator;
use timing::BenchMark;

pub struct Hit {
    hit_info: HitInfo,
    geometry_index: usize,
    vertex_index: usize,
}
impl Hit {
    fn new(hit_info: HitInfo, geometry_index: usize, vertex_index: usize) -> Self {
        Hit {
            hit_info,
            geometry_index,
            vertex_index,
        }
    }
}

pub struct RayTracer<Accel = OctTreeIntersector>
where
    Accel: Intersector,
{
    width: usize,
    height: usize,
    pub camera: Camera,

    sample_generator: sample_generator::SampleGenerator,
    pub film: Film,
    accel: Accel,

    current_row: usize,
}

impl<Accel> RayTracer<Accel> 
where Accel: Intersector {
    
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, camera: Camera, scene: &Scene) -> Self {
        RayTracer {
            width,
            height,
            camera,
            sample_generator: SampleGenerator::new(),
            film: Film::new(width * height),
            accel: Intersector::new(scene),
            current_row: 0,
        }
    }

    pub fn new_with_intersector(width: usize, height: usize, camera: Camera, accel: Accel) -> Self {
        RayTracer {
            width,
            height,
            camera,
            sample_generator: SampleGenerator::new(),
            film: Film::new(width * height),
            accel,
            current_row: 0,
        }
    }

    pub fn trace_frame_additive(&mut self, scene: &Scene, _timer: &mut BenchMark) -> u32 {
        const RECURSIONS: u8 = 2;
        const SUB_SPREAD: u32 = 1;

        let mut rng = rand::thread_rng();

        let mut num_primary_rays = 0;
        for _ in 0..50 {
            for (i, pixel_data) in self.film.pixel_datas[self.current_row*self.width..(self.current_row+1)*self.width].iter_mut().enumerate()
            {
                let idx = self.current_row*self.width + i;
                let ray = self
                    .camera
                    .get_ray(idx % self.width, idx / self.height, &mut rng);

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
                pixel_data.add_sample(color);
            }
            num_primary_rays += self.width as u32;
            self.current_row = (self.current_row + 1) % self.height;
        }
        return num_primary_rays;
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
    let radiance = shade(accel, scene, ray, hit, &normal);
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
    let hit_point = ray.pos + hit.hit_info.t * ray.dir;
    let hit_point = hit_point + 0.00001 * &random_dir;

    Ray::new(hit_point, random_dir)
}

fn calc_normal(scene: &Scene, hit: &Hit) -> Vec3 {
    let geom_vertices = &scene.geometries[hit.geometry_index].vertices;
    let normal = cross(
        &(geom_vertices[hit.vertex_index + 1] - geom_vertices[hit.vertex_index]),
        &(geom_vertices[hit.vertex_index + 2] - geom_vertices[hit.vertex_index]),
    );
    normal.normalized()
}

fn shade<Accel>(accel: &Accel, scene: &Scene, ray: &Ray, hit: &Hit, normal: &Vec3) -> RGB 
where Accel: Intersector {
    let mut accum_color = RGB::black();
    let hit_point = ray.pos + hit.hit_info.t * ray.dir;

    for light in &scene.lights {
        let ray_to_light = Ray::new(hit_point, light.pos - hit_point);
        let dot_light_normal = dot(&normal, &ray_to_light.dir.normalized());

        if dot_light_normal < 0.0 {
            continue; // triangle is facing away from light
        }

        //is light blocked by geometry?
        let mut blocked = false;
        let ray_to_light_offseted = Ray::new(ray_to_light.pos + ray_to_light.dir * 0.01, ray_to_light.dir);
        if let Some(hit) = accel.intersect_ray(&scene, &ray_to_light_offseted) {
            if hit.hit_info.t > 0.01 && hit.hit_info.t < 1.0 {
                blocked = true;
            }
        }
        
        // for geom in &scene.geometries {
        //     for tri_vertices in geom.vertices.chunks(3) {
        //         if let Some(t) = intersect::intersect_later_out( 
        //             &ray_to_light,
        //             &tri_vertices[0],
        //             &tri_vertices[1],
        //             &tri_vertices[2],
        //         ) {
        //             if t > 0.0001 && t < 1.0 {
        //                 blocked = true;
        //                 break;
        //             }
        //         }
        //     }
        // }

        if !blocked {
            //lambertian / diffuse
            // accum_color += dot_light_normal
            //     * light.color
            //     * scene.geometries[hit.geometry_index].material.diffuse;

            // phong
            {
                const SPECULAR: RGB = RGB::white();
                let diffuse = &scene.geometries[hit.geometry_index].material.diffuse;
                let diffuse_rgb = match diffuse {
                    Diffuse::Color(rgb) => rgb,
                    Diffuse::TextureId(tex_id) => {
                        let texture = &scene.textures[*tex_id];
                        texture.get_texel(hit.hit_info.u, hit.hit_info.v)
                    }
                };


                const SHININESS: f32 = 32.0;
                let view_ray = ray.dir.normalized();
                let reflected_light = 2.0*dot_light_normal*normal - ray_to_light.dir.normalized();
                accum_color += (diffuse_rgb*dot_light_normal + SPECULAR*dot(&view_ray, &reflected_light).powf(SHININESS)) * light.color;
            }
        }
    }
    accum_color
}
