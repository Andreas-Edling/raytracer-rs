
use crate::vecmath::{cross, dot};

use crate::scene::{
    Scene,
    Vertex,
    color::RGB,
    Light,
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
    vertex_index: usize,
}
impl Hit {
    fn new(distance: f32, vertex_index: usize) -> Self {
        Hit{ distance, vertex_index}
    }
}

pub struct RayTracer {
    pub scene: Scene,
    width: usize,
    height: usize,
    pub camera: Camera,
}


impl RayTracer {
    pub fn new(width: usize, height: usize, scene: Scene) -> Self {
        let camera = scene.cameras[0].clone();
        RayTracer {
            scene,
            width,
            height,
            camera,
        }
    }

    #[rustfmt::skip]
    pub fn trace_frame(&mut self) -> Vec<RGB> {
        let rays = self.camera.get_rays();
        let mut frame = Vec::with_capacity(self.width*self.height);

        let mut intersect_vals = vec![None;self.scene.transformed_vertices.len()/3];

        for ray in rays {
            let mut closest_hit = None;

            for (i,tri_vertices) in self.scene.transformed_vertices.chunks(3).enumerate() {
                intersect_vals[i] = intersect_late_out(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]);
            }

            for (i, intersect_val) in intersect_vals.iter().enumerate() {
                match (&closest_hit, intersect_val) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(dist)) => {
                        closest_hit = Some(Hit::new(*dist, i*3));
                    },
                    (Some(hit), Some(dist)) => {
                        if  *dist < hit.distance {
                            closest_hit = Some(Hit::new(*dist, i*3));
                        }
                    },
                }
            }

            let color = match closest_hit {
                Some(ref hit) => {
                    let rgb = RayTracer::shade(ray, hit, &self.scene.lights, &self.scene.transformed_vertices);
                    rgb
                },
                None => RGB::black()
            };
            frame.push(color);
        }

        frame
    }

    fn shade(ray: &Ray, hit: &Hit, lights: &[Light], vertices: &[Vertex]) -> RGB {
        let mut accum_color = RGB::new(1.0, 0.0, 0.0);
        let hit_point = &ray.pos + hit.distance * &ray.dir;

        for light in lights {
            let ray_to_light = Ray::new(hit_point.clone(), &light.pos - &hit_point);

            let normal = cross( 
                &(&vertices[hit.vertex_index+1] - &vertices[hit.vertex_index]), 
                &(&vertices[hit.vertex_index+2] - &vertices[hit.vertex_index]));
            let normal = normal.normalized();

            let dot_light_normal = dot(&normal, &ray_to_light.dir.normalized());
        
            if dot_light_normal < 0.0 {
                continue;  // triangle is facing away from light
            }

            //is light blocked by (other) geometry?
            let mut blocked = false;
            for tri_vertices in vertices.chunks(3) {
                if let Some(t) = intersect_later_out(&ray_to_light, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]) {
                    if t > 0.0001 && t < 1.0 {
                        blocked = true;
                        break;
                    }
                }
            };

            if !blocked {
                //lambertian / diffuse
                //accum_color += dot_light_normal * &light.color; 

                // phong
                {
                    const SPECULAR: f32 = 0.5;
                    const DIFFUSE: f32 = 0.5;
                    const SHININESS: f32 = 32.0;
                    let view_ray = -ray.dir.normalized(); 
                    let reflected_light = 2.0*dot_light_normal*normal - ray_to_light.dir.normalized();
                    accum_color += (DIFFUSE*dot_light_normal + SPECULAR*dot(&view_ray, &reflected_light).powf(SHININESS)) * light.color;
                }
            }
        }
        accum_color
    }
}
