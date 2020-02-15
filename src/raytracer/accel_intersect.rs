use super::{intersect::intersect_late_out, Hit};
use crate::scene::Scene;
use crate::vecmath::{cross, dot, Ray, Vec3};

pub trait Intersector {
    fn new(scene: &Scene) -> Self;
    fn intersect_ray(&self, scene: &Scene, ray: &Ray) -> Option<Hit>;
}

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
                    intersect_late_out(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2])
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

#[derive(Clone, Copy)]
struct TriangleIndex {
    geom_idx: usize,
    tri_idx: usize,
}
impl TriangleIndex {
    pub fn new(geom_idx: usize, tri_idx: usize) -> Self {
        TriangleIndex { geom_idx, tri_idx }
    }
}
#[derive(Clone)]
pub struct Cube {
    min: Vec3,
    max: Vec3,
}
impl Cube {
    fn new(min: Vec3, max: Vec3) -> Self {
        Cube { min, max }
    }
}

struct Leaf {
    cube_index: usize,
    triangle_indices: Vec<TriangleIndex>,
}
impl Leaf {
    pub fn new(cube_index: usize, triangle_indices: Vec<TriangleIndex>) -> Self {
        Leaf {
            cube_index,
            triangle_indices,
        }
    }
}
enum OctNode {
    Leaf(Leaf),
    Node([usize; 8]), //indices to nodes & cubes vec
}

pub struct OctTreeAccelerationIntersector {
    cubes: Vec<Cube>,
    nodes: Vec<OctNode>,
    trunk: usize,
}
impl OctTreeAccelerationIntersector {
    fn split(&mut self, num_triangles: usize, scene: &Scene) {
        Self::split_node(
            self.trunk,
            &mut self.nodes,
            &mut self.cubes,
            num_triangles,
            scene,
        );
    }

    fn split_node(
        node_idx: usize,
        nodes: &mut Vec<OctNode>,
        cubes: &mut Vec<Cube>,
        num_triangles: usize,
        scene: &Scene,
    ) {
        let mut new_nodes = Vec::new();
        let mut new_child_list = [0; 8];

        match nodes[node_idx] {
            OctNode::Node(_) => return,
            OctNode::Leaf(ref leaf) => {
                if leaf.triangle_indices.len() <= num_triangles {
                    return;
                }

                println!(
                    "splitting node {} containing {} triangles",
                    node_idx,
                    leaf.triangle_indices.len()
                );

                let cube = &cubes[leaf.cube_index];
                let child_cubes = generate_child_cubes(cube);
                for i in 0..8 {
                    let triangles_inside = triangles_intersecting_cube(
                        &child_cubes[i],
                        &leaf.triangle_indices,
                        &scene,
                    );
                    cubes.push(child_cubes[i].clone());
                    let child_idx = cubes.len() - 1;
                    new_nodes.push(OctNode::Leaf(Leaf::new(child_idx, triangles_inside)));
                    new_child_list[i] = child_idx;
                }
            }
        }
        nodes[node_idx] = OctNode::Node(new_child_list);
        let start_idx_new_nodes = nodes.len();
        nodes.append(&mut new_nodes);
        for child_node_idx in start_idx_new_nodes..start_idx_new_nodes + 8 {
            Self::split_node(child_node_idx, nodes, cubes, num_triangles, &scene);
        }
    }

    fn intersect_node(
        &self,
        scene: &Scene,
        ray: &Ray,
        inv_ray: &Ray,
        node_idx: usize,
    ) -> Option<Hit> {
        match self.nodes[node_idx] {
            OctNode::Leaf(ref leaf) => return intersect_leaf_triangles(scene, ray, leaf),

            OctNode::Node(ref child_indices) => {
                // check children for intersections and order them
                let mut distances = Vec::new();
                for child_index in child_indices {
                    if let Some(t) = intersect_cube_inverse_ray(inv_ray, &self.cubes[*child_index])
                    {
                        distances.push((*child_index, t));
                    }
                }
                distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let ordered_child_indices =
                    distances.iter().map(|(idx, _t)| *idx).collect::<Vec<_>>();

                // recurse
                for child_idx in ordered_child_indices {
                    if let Some(hit) = self.intersect_node(scene, ray, inv_ray, child_idx) {
                        return Some(hit);
                    }
                }

                None
            }
        }
    }

    fn print_debug_info(&self) {
        self.print_rec(self.trunk, 0)
    }

    fn print_rec(&self, node_idx: usize, recurse_level: usize) {
        let node = &self.nodes[node_idx];
        match node {
            OctNode::Node(child_indices) => {
                (0..recurse_level).for_each(|_| print!("    "));
                println!("node {}", node_idx);
                for child_idx in child_indices {
                    self.print_rec(*child_idx, recurse_level + 1);
                }
            }
            OctNode::Leaf(leaf) => {
                (0..recurse_level).for_each(|_| print!("    "));
                println!(
                    "leaf {} contains {} triangles",
                    node_idx,
                    leaf.triangle_indices.len()
                );
            }
        }
    }
}

impl Intersector for OctTreeAccelerationIntersector {
    fn new(scene: &Scene) -> Self {
        let trunk_cube = calc_extents(&scene);
        let all_triangle_indices = all_triangle_indices(&scene);
        let trunk = 0;
        let leaf = Leaf::new(trunk, all_triangle_indices);
        let nodes = vec![OctNode::Leaf(leaf)];
        let mut octtree = OctTreeAccelerationIntersector {
            cubes: vec![trunk_cube],
            nodes,
            trunk,
        };

        octtree.split(50, scene);
        octtree.print_debug_info();
        octtree
    }

    fn intersect_ray(&self, scene: &Scene, ray: &Ray) -> Option<Hit> {
        let inv_ray = Ray::new(
            ray.pos,
            Vec3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z),
        );
        return self.intersect_node(scene, ray, &inv_ray, self.trunk);
    }
}

fn intersect_leaf_triangles(scene: &Scene, ray: &Ray, leaf: &Leaf) -> Option<Hit> {
    let mut closest_hit = None;

    for index in &leaf.triangle_indices {
        let tri_vertices = &scene.geometries[index.geom_idx].transformed_vertices
            [index.tri_idx..index.tri_idx + 3];
        let t = intersect_late_out(ray, &tri_vertices[0], &tri_vertices[1], &tri_vertices[2]);
        match (&closest_hit, t) {
            (None, None) => (),
            (Some(_), None) => (),
            (None, Some(dist)) => closest_hit = Some(Hit::new(dist, index.geom_idx, index.tri_idx)),
            (Some(hit), Some(dist)) => {
                if dist < hit.distance {
                    closest_hit = Some(Hit::new(dist, index.geom_idx, index.tri_idx))
                }
            }
        }
    }
    closest_hit
}

fn generate_child_cubes(cube: &Cube) -> [Cube; 8] {
    let mid = 0.5 * (cube.max + cube.min);
    let min = cube.min.clone();
    let max = cube.max.clone();

    [
        Cube::new(
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(mid.x, mid.y, mid.z),
        ),
        Cube::new(
            Vec3::new(mid.x, min.y, min.z),
            Vec3::new(max.x, mid.y, mid.z),
        ),
        Cube::new(
            Vec3::new(min.x, mid.y, min.z),
            Vec3::new(mid.x, max.y, mid.z),
        ),
        Cube::new(
            Vec3::new(mid.x, mid.y, min.z),
            Vec3::new(max.x, max.y, mid.z),
        ),
        Cube::new(
            Vec3::new(min.x, min.y, mid.z),
            Vec3::new(mid.x, mid.y, max.z),
        ),
        Cube::new(
            Vec3::new(mid.x, min.y, mid.z),
            Vec3::new(max.x, mid.y, max.z),
        ),
        Cube::new(
            Vec3::new(min.x, mid.y, mid.z),
            Vec3::new(mid.x, max.y, max.z),
        ),
        Cube::new(
            Vec3::new(mid.x, mid.y, mid.z),
            Vec3::new(max.x, max.y, max.z),
        ),
    ]
}

fn calc_extents(scene: &Scene) -> Cube {
    let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
    let mut max = Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);

    for geom in &scene.geometries {
        for vtx in &geom.transformed_vertices {
            min.x = min.x.min(vtx.x);
            min.y = min.y.min(vtx.y);
            min.z = min.z.min(vtx.z);
            max.x = max.x.max(vtx.x);
            max.y = max.y.max(vtx.y);
            max.z = max.z.max(vtx.z);
        }
    }
    Cube { min, max }
}

fn all_triangle_indices(scene: &Scene) -> Vec<TriangleIndex> {
    scene
        .geometries
        .iter()
        .enumerate()
        .flat_map(|(geom_idx, geom)| {
            (0..geom.transformed_vertices.len() / 3)
                .map(move |vtx_idx| TriangleIndex::new(geom_idx, vtx_idx * 3))
        })
        .collect::<Vec<_>>()
}


// returns t as dist along closest axis.
// returns negative t if origin is inside cube.
// None for no intersection.
// Ray direction has to be inversed! (1.0/dir)
pub fn intersect_cube_inverse_ray(inv_ray: &Ray, cube: &Cube) -> Option<f32> {
    let tx1 = (cube.min.x - inv_ray.pos.x) * inv_ray.dir.x;
    let tx2 = (cube.max.x - inv_ray.pos.x) * inv_ray.dir.x;

    let tmin = tx1.min(tx2);
    let tmax = tx1.max(tx2);

    let ty1 = (cube.min.y - inv_ray.pos.y) * inv_ray.dir.y;
    let ty2 = (cube.max.y - inv_ray.pos.y) * inv_ray.dir.y;

    let tmin = tmin.max(ty1.min(ty2));
    let tmax = tmax.min(ty1.max(ty2));

    let tz1 = (cube.min.z - inv_ray.pos.z) * inv_ray.dir.z;
    let tz2 = (cube.max.z - inv_ray.pos.z) * inv_ray.dir.z;

    let tmin = tmin.max(tz1.min(tz2));
    let tmax = tmax.min(tz1.max(tz2));

    return if tmax >= tmin && tmax > 0.0 {
        Some(tmin)
    } else {
        None
    };
}

fn triangles_intersecting_cube(
    cube: &Cube,
    triangle_indices: &Vec<TriangleIndex>,
    scene: &Scene,
) -> Vec<TriangleIndex> {
    let mut insiders = Vec::new();
    for indices in triangle_indices {
        if triangle_cube_intersection(
            cube,
            &scene.geometries[indices.geom_idx].transformed_vertices
                [indices.tri_idx..indices.tri_idx + 3],
        )
        {
            insiders.push(indices.clone());
            continue;
        }
    }
    insiders
}

fn triangle_cube_intersection(cube: &Cube, tri_vertices: &[Vec3]) -> bool {
    //based on SAT (Separating axis theorem)

    // cube normals as axes test
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let z_axis = Vec3::new(0.0, 0.0, 1.0);

    let (tri_min, tri_max) = project_points_on_axis(tri_vertices, &x_axis);
    if tri_max < cube.min.x || tri_min > cube.max.x {
        return false;
    }
    let (tri_min, tri_max) = project_points_on_axis(tri_vertices, &y_axis);
    if tri_max < cube.min.y || tri_min > cube.max.y {
        return false;
    }
    let (tri_min, tri_max) = project_points_on_axis(tri_vertices, &z_axis);
    if tri_max < cube.min.z || tri_min > cube.max.z {
        return false;
    }

    // triangle normal axis test
    let cube_vertices = [
        cube.min.clone(),
        Vec3::new(cube.max.x, cube.min.y, cube.min.z),
        Vec3::new(cube.min.x, cube.max.y, cube.min.z),
        Vec3::new(cube.min.x, cube.min.y, cube.max.z),
        Vec3::new(cube.min.x, cube.max.y, cube.max.z),
        Vec3::new(cube.max.x, cube.min.y, cube.max.z),
        Vec3::new(cube.max.x, cube.max.y, cube.min.z),
        cube.max.clone(),
    ];
    let tri_edge_1 = tri_vertices[0] - tri_vertices[1];
    let tri_edge_2 = tri_vertices[1] - tri_vertices[2];
    let tri_normal = cross(&tri_edge_1, &tri_edge_2);
    let tri_offset = dot(&tri_normal, &tri_vertices[0]);
    let (cube_min, cube_max) = project_points_on_axis(&cube_vertices, &tri_normal);
    if cube_max < tri_offset || cube_min > tri_offset {
        return false;
    }

    let tri_edge_3 = tri_vertices[2] - tri_vertices[0];

    // Test the nine edge cross-products
    let axes = [
        cross(&tri_edge_1, &x_axis),
        cross(&tri_edge_1, &y_axis),
        cross(&tri_edge_1, &z_axis),
        cross(&tri_edge_2, &x_axis),
        cross(&tri_edge_2, &y_axis),
        cross(&tri_edge_2, &z_axis),
        cross(&tri_edge_3, &x_axis),
        cross(&tri_edge_3, &y_axis),
        cross(&tri_edge_3, &z_axis),
    ];
    for axis in &axes {
        let (cube_min, cube_max) = project_points_on_axis(&cube_vertices, axis);
        let (tri_min, tri_max) = project_points_on_axis(tri_vertices, axis);
        if cube_max < tri_min || cube_min > tri_max {
            return false;
        }
    }

    // no separating axis found
    return true;
}

fn project_points_on_axis(points: &[Vec3], axis: &Vec3) -> (f32, f32) {
    let mut min = std::f32::MAX;
    let mut max = std::f32::MIN;
    for point in points {
        let val = dot(axis, point);
        min = min.min(val);
        max = max.max(val);
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_cube_inverse_ray() {
        let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let dir = Vec3::new(-1.0, 0.1, 0.1);
        let inv_dir = Vec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);
        let ray = Ray::new(Vec3::new(2.0, 0.0, 0.0), inv_dir);
        let t = intersect_cube_inverse_ray(&ray, &cube).unwrap();
        assert_eq!(t, 1.0);
    }
    #[test]
    fn test_intersect_cube_inverse_ray_handles_inf() {
        let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let dir = Vec3::new(-1.0, 0.0, 0.0);
        let inv_dir = Vec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z); //division by zero here, by design
        let ray = Ray::new(Vec3::new(2.0, 0.0, 0.0), inv_dir);
        let t = intersect_cube_inverse_ray(&ray, &cube).unwrap();
        assert_eq!(t, 1.0);
    }

    #[test]
    fn test_intersect_cube_inverse_ray_start_inside() {
        let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let dir = Vec3::new(1.0, 0.1, 0.1);
        let inv_dir = Vec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);
        let ray = Ray::new(Vec3::new(-0.9, 0.0, 0.0), inv_dir);
        let t = intersect_cube_inverse_ray(&ray, &cube).unwrap();
        assert!(t < 0.0);
    }

    #[test]
    fn test_intersect_cube_inverse_ray_should_miss() {
        let cube = Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let dir = Vec3::new(-1.0, 0.1, 0.1);
        let inv_dir = Vec3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);
        let ray = Ray::new(Vec3::new(-2.0, 0.0, 0.0), inv_dir);
        let t = intersect_cube_inverse_ray(&ray, &cube);
        assert!(t.is_none());
    }
}
