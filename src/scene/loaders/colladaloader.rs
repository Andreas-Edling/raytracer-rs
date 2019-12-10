use parseval::{
    xml,
    parsers::*,
};

use crate::scene::{
    Scene, 
    Vertex,
    color::RGB,
    Light,
    camera::Camera,
    Vec3,
};

mod collada_matrix;
use collada_matrix::ColladaMatrix;

pub use super::SceneLoader;

pub struct ColladaLoader {}

impl SceneLoader for ColladaLoader {
    fn from_str(doc: &str) -> Result<Scene, String> {
        let collada = Collada::parse(doc)?;
        collada.to_scene_flatten()
    }

    fn load() -> Result<Scene, String> {
        Self::from_str(COLLADA_DOC)
    }
}




pub struct Collada {
    cameras: Vec<ColladaCamera>,
    lights: Vec<ColladaLight>,
    _effects: Vec<ColladaEffect>,
    _materials: Vec<ColladaMaterial>,
    geometries: Vec<ColladaGeometry>,
    visual_scenes: Vec<ColladaVisualScene>
}
impl Collada {
    pub fn parse(input: &str) -> Result<Collada, String> {
        let (remaining, _xml_version) = xml::xml_definition_element().parse(input)?;  //<?xml version="1.0" encoding="utf-8"?>
        let (remaining, collada_elem) = xml::opening_element().parse(remaining)?;
        if collada_elem.name != "COLLADA" { return Err("not a collada doc".to_string()); }
        let (remaining, _asset_element) = xml::element_with_name("asset".to_string()).parse(remaining)?;
        let (remaining, cameras_element) = xml::element_with_name("library_cameras".to_string()).parse(remaining)?;
        let (remaining, lights_element) = xml::element_with_name("library_lights".to_string()).parse(remaining)?;
        let (remaining, effects_element) = xml::element_with_name("library_effects".to_string()).parse(remaining)?;
        let (remaining, _images_element) = xml::element_with_name("library_images".to_string()).parse(remaining)?;
        let (remaining, materials_element) = xml::element_with_name("library_materials".to_string()).parse(remaining)?;
        let (remaining, geometries_element) = xml::element_with_name("library_geometries".to_string()).parse(remaining)?;
        let (remaining, visual_scenes_element) = xml::element_with_name("library_visual_scenes".to_string()).parse(remaining)?;
        let (remaining, _scene_element) = xml::element_with_name("scene".to_string()).parse(remaining)?;
        let (remaining, _) = xml::closing_element("COLLADA".to_string()).parse(remaining)?;

        assert_eq!(remaining.len(),0);

        let cameras = to_cameras(&cameras_element)?;
        let lights = to_lights(&lights_element)?;
        let _effects = to_effects(&effects_element)?;
        let _materials = to_materials(&materials_element)?;
        let geometries = to_geometries(&geometries_element)?;
        let visual_scenes = to_visual_scenes(&visual_scenes_element)?;

        Ok( 
            Collada {
                cameras,
                lights,
                _effects,
                _materials,
                geometries,
                visual_scenes,
            }
        )
    }

    pub fn to_scene_flatten(&self) -> Result<Scene, String> {
        let mut vertices = vec![];
        let mut lights = vec![];
        let mut cameras = vec![];

        for visual_scene in &self.visual_scenes {
            for node in &visual_scene.nodes {
                println!("imported node {}",node.id);
                for camera in &self.cameras {
                    if camera.id != node.id {
                        continue;
                    }
                    println!("collada cam matrix {:?}", node.matrix);
                    cameras.push(Camera::from_orientation_matrix(640, 480, &node.matrix.to_vecmath_matrix(), camera.fov));
                }

                for light in &self.lights {
                    if light.id != node.id {
                        continue;
                    }
                    let transformed_light_pos = crate::vecmath::Vec3::from(&node.matrix.to_vecmath_matrix() * crate::vecmath::Vec4::from_vec3(&light.light.pos));
                    lights.push(Light::new(transformed_light_pos, light.light.color));
                }

                for geometry in &self.geometries {
                    if geometry.id != node.id {
                        continue;
                    }

                    let mut geom_vertices = vec![];
                    for tri_vtx_indices in geometry.triangles.chunks(3) {
                        geom_vertices.push(Vertex::new( 
                            geometry.vertices[3*tri_vtx_indices[0] as usize],
                            geometry.vertices[3*tri_vtx_indices[0] as usize + 1],
                            geometry.vertices[3*tri_vtx_indices[0] as usize + 2],
                        ));
                        geom_vertices.push(Vertex::new( 
                            geometry.vertices[3*tri_vtx_indices[1] as usize],
                            geometry.vertices[3*tri_vtx_indices[1] as usize + 1],
                            geometry.vertices[3*tri_vtx_indices[1] as usize + 2],
                        ));
                        geom_vertices.push(Vertex::new( 
                            geometry.vertices[3*tri_vtx_indices[2] as usize],
                            geometry.vertices[3*tri_vtx_indices[2] as usize + 1],
                            geometry.vertices[3*tri_vtx_indices[2] as usize + 2],
                        ));
                    }

                    for vertex in geom_vertices.iter() {
                        let transformed = crate::vecmath::Vec3::from(&node.matrix.to_vecmath_matrix() * crate::vecmath::Vec4::from_vec3(vertex));
                        vertices.push(transformed);
                    }
                    break;
                }
            }
        }

        let transformed_vertices = vertices.clone();

        Ok(Scene {
            vertices,
            lights,
            transformed_vertices,
            cameras,
        })
    }
}

struct _ColladaAsset;

struct ColladaCamera {
    id: String,
    fov: f32,
    _aspect_ratio: f32,
    _scene_matrix: crate::vecmath::Matrix,
}

struct ColladaLight {
    id: String,
    light: Light,
}

struct ColladaEffect;

struct _ColladaImage;

struct ColladaMaterial;

struct ColladaGeometry {
    vertices: Vec<f32>,
    triangles: Vec<u32>,
    id: String,
}
struct ColladaVisualScene {
    nodes: Vec<VisualSceneNode>,
}

struct _ColladaScene;


struct VisualSceneNode {
    id: String,
    matrix: ColladaMatrix,
}

impl VisualSceneNode {
    pub fn new(id: String, matrix: ColladaMatrix) -> Self {
        VisualSceneNode {
            id,
            matrix,
        }
    }
}



fn convert_geometry(geometry_element: &xml::Element) -> Result<ColladaGeometry, String> {
    let id = geometry_element.get_attrib_value("id")?;
    let mesh = geometry_element.get_child_by_name("mesh")?;

    // get vertex positions
    let mut vertices = vec![];
    let positions_array = mesh
        .get_child_by_attrib(("id", format!("{}-positions",id)))?
        .get_child_by_attrib(("id", format!("{}-positions-array",id)))?;

    if let xml::DataOrElements::Data(vertices_str) = &positions_array.data_or_elements {
        let (_, parsed_vertices) = array_f32().parse(vertices_str)?;
        vertices = parsed_vertices;
    }

    // get triangle indices
    let mut triangles = vec![];
    let index_array = mesh
        .get_child_by_name("triangles")?
        .get_child_by_name("p")?;

    if let xml::DataOrElements::Data(triangle_indices_str) = &index_array.data_or_elements {
        let (_, parsed_index_array) = array_u32().parse(triangle_indices_str)?;
        for (pos_index, _normal_index, _texcoord_index) in parsed_index_array.chunks(3).map(|indices| (indices[0], indices[1], indices[2])) {
            triangles.push(pos_index);
        }
    }

    Ok(
        ColladaGeometry {
            vertices,
            triangles,
            id: id.to_string(),
        }
    )
}

fn to_cameras(elem: &xml::Element) -> Result<Vec<ColladaCamera>, String> {
    if let xml::DataOrElements::Elements(camera_elements) = &elem.data_or_elements {
        let mut cameras = vec![];
        for camera_elem in camera_elements {
            let id = camera_elem.get_attrib_value("id")?.to_string();
            let perspective_elem = camera_elem
                .get_child_by_name("optics")?
                .get_child_by_name("technique_common")?
                .get_child_by_name("perspective")?;

            let fov_elem = perspective_elem.get_child_by_name("xfov")?;
            let aspect_ratio_elem = perspective_elem.get_child_by_name("aspect_ratio")?;
            let fov = match &fov_elem.data_or_elements {
                xml::DataOrElements::Data(fov_data) =>{ let (_,fov) = array_f32().parse(fov_data)?; fov },
                _ => return Err("cant read fov".to_string()),
            }[0];

            let _aspect_ratio = match &aspect_ratio_elem.data_or_elements {
                xml::DataOrElements::Data(aspect_ratio_data) =>{ let (_,aspect_ratio) = array_f32().parse(aspect_ratio_data)?; aspect_ratio },
                _ => return Err("cant read aspect_ratio".to_string()),
            }[0];

            cameras.push(
                ColladaCamera { 
                    id, 
                    fov, 
                    _aspect_ratio, 
                    _scene_matrix: crate::vecmath::Matrix::ident(), // will be gotten from VisualSceneNode
                }
            );
        }
        return Ok(cameras);
    }
    Err("cant convert cameras".to_string())
}

fn to_lights(elem: &xml::Element) -> Result<Vec<ColladaLight>, String> {
    if let xml::DataOrElements::Elements(light_elements) = &elem.data_or_elements {
        let mut lights = vec![];
        for light_elem in light_elements {

            let id = light_elem.get_attrib_value("id")?.to_string();
            let color = {
                let color_elem = light_elem
                    .get_child_by_name("technique_common")?
                    .get_child_by_name("point")?
                    .get_child_by_name("color")?;

                match &color_elem.data_or_elements {
                    xml::DataOrElements::Data(color_data) =>{ let (_, color_array) = array_f32().parse(color_data)?; Ok(color_array)},
                    xml::DataOrElements::Elements(_) => Err("cant get color".to_string()),
                }
            }?;

            let color = RGB::new(color[0], color[1], color[2]);
            let pos = Vec3::new(0.0,0.0,0.0); //transform with position is found in visualScenes element
            lights.push(ColladaLight{ id, light: Light::new(pos, color)});
        }
        return Ok(lights);
    }
    Err("cant convert lights".to_string())
}

fn to_effects(_elem: &xml::Element) -> Result<Vec<ColladaEffect>, String> {
    Ok(vec![])
}

fn to_materials(_elem: &xml::Element) -> Result<Vec<ColladaMaterial>, String> {
    Ok(vec![])
}

fn to_geometries(elem: &xml::Element) -> Result<Vec<ColladaGeometry>, String> {
    if let xml::DataOrElements::Elements(geometry_elements) = &elem.data_or_elements {
        let mut geometries = vec![];
        for geometry_elem in geometry_elements {
            geometries.push(convert_geometry(geometry_elem)?);
        }
        return Ok(geometries);
    }
    Err("cant convert geometries".to_string())
}

fn to_visual_scenes(elem: &xml::Element) -> Result<Vec<ColladaVisualScene>, String> {
    if let xml::DataOrElements::Elements(scenes) = &elem.data_or_elements {
        let mut nodes = vec![];
        for scene in scenes {
            if let xml::DataOrElements::Elements(node_elements) = &scene.data_or_elements {
                for node_elem in node_elements {

                    let name = match (
                        node_elem.get_child_by_name("instance_light"), 
                        node_elem.get_child_by_name("instance_geometry"), 
                        node_elem.get_child_by_name("instance_camera") 
                    ){
                        (Ok(instance_light), _, _) => &instance_light.get_attrib_value("url")?[1..], //strip '#' with [1..]
                        (_, Ok(instance_geom), _) => &instance_geom.get_attrib_value("url")?[1..],
                        (_, _, Ok(instance_cam)) =>  &instance_cam.get_attrib_value("url")?[1..],
                        _ => return Err("unsupported node type".to_string()),
                    };

                    let matrix_elem = node_elem.get_child_by_name("matrix")?;
                    if let xml::DataOrElements::Data(matrix_data) = &matrix_elem.data_or_elements {
                        let (_,matrix_array) = array_f32().parse(matrix_data)?;
                        let collada_matrix = ColladaMatrix::from_slice(&matrix_array[..]).ok_or("cant create array".to_string())?;
                        nodes.push(VisualSceneNode::new(name.to_string(), collada_matrix));
                    }
                }
            }
        }
        return Ok(
            vec![
                ColladaVisualScene {
                    nodes
                }
            ]
        );
    }

    Err("No scene element(s)".to_string())
}


#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn collada_mat_to_vecmat_translation() {
        let cm = ColladaMatrix::from_slice(&[
            0.0, 0.0, 0.0, 10.0,
            0.0, 0.0, 0.0, 20.0,
            0.0, 0.0, 0.0, 30.0,
            0.0, 0.0, 0.0, 1.0,
        ]).unwrap();
        let m = cm.to_vecmath_matrix();
        let expected = crate::vecmath::Matrix::new(&[
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            10.0, 30.0, 20.0, 1.0,
        ]);
        assert_eq!(m, expected);
    }

    #[test]
    fn collada_mat_to_vecmat_z_vec() {
        let cm = ColladaMatrix::from_slice(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).unwrap();
        let m = cm.to_vecmath_matrix();
        
        let z_vec = Vec3::new(0.0,0.0,1.0);
        let expected = Vec3::new(0.0,-1.0,0.0);

        let actual = Vec3::from(m * crate::vecmath::Vec4::from_vec3(&z_vec));

        assert_eq!(actual, expected);
    }


    #[test]
    fn test_parse() {
        let parsed = Collada::parse(COLLADA_DOC);
        match &parsed {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
        assert!(parsed.is_ok());
    }
}

const COLLADA_DOC: &str = r##"<?xml version="1.0" encoding="utf-8"?>
    <COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <asset>
        <contributor>
        <author>Blender User</author>
        <authoring_tool>Blender 2.80.75 commit date:2019-07-29, commit time:14:47, hash:f6cb5f54494e</authoring_tool>
        </contributor>
        <created>2019-11-02T06:46:27</created>
        <modified>2019-11-02T06:46:27</modified>
        <unit name="meter" meter="1"/>
        <up_axis>Z_UP</up_axis>
    </asset>
    <library_cameras>
        <camera id="Camera-camera" name="Camera">
        <optics>
            <technique_common>
            <perspective>
                <xfov sid="xfov">39.59775</xfov>
                <aspect_ratio>1.777778</aspect_ratio>
                <znear sid="znear">0.1</znear>
                <zfar sid="zfar">100</zfar>
            </perspective>
            </technique_common>
        </optics>
        <extra>
            <technique profile="blender">
            <shiftx sid="shiftx" type="float">0</shiftx>
            <shifty sid="shifty" type="float">0</shifty>
            <dof_distance sid="dof_distance" type="float">10</dof_distance>
            </technique>
        </extra>
        </camera>
    </library_cameras>
    <library_lights>
        <light id="Light-light" name="Light">
        <technique_common>
            <point>
            <color sid="color">1000 1000 1000</color>
            <constant_attenuation>1</constant_attenuation>
            <linear_attenuation>0</linear_attenuation>
            <quadratic_attenuation>0.00111109</quadratic_attenuation>
            </point>
        </technique_common>
        <extra>
            <technique profile="blender">
            <type sid="type" type="int">0</type>
            <flag sid="flag" type="int">0</flag>
            <mode sid="mode" type="int">1</mode>
            <gamma sid="blender_gamma" type="float">1</gamma>
            <red sid="red" type="float">1</red>
            <green sid="green" type="float">1</green>
            <blue sid="blue" type="float">1</blue>
            <shadow_r sid="blender_shadow_r" type="float">0</shadow_r>
            <shadow_g sid="blender_shadow_g" type="float">0</shadow_g>
            <shadow_b sid="blender_shadow_b" type="float">0</shadow_b>
            <energy sid="blender_energy" type="float">1000</energy>
            <dist sid="blender_dist" type="float">29.99998</dist>
            <spotsize sid="spotsize" type="float">75</spotsize>
            <spotblend sid="spotblend" type="float">0.15</spotblend>
            <att1 sid="att1" type="float">0</att1>
            <att2 sid="att2" type="float">1</att2>
            <falloff_type sid="falloff_type" type="int">2</falloff_type>
            <clipsta sid="clipsta" type="float">0.04999995</clipsta>
            <clipend sid="clipend" type="float">30.002</clipend>
            <bias sid="bias" type="float">1</bias>
            <soft sid="soft" type="float">3</soft>
            <bufsize sid="bufsize" type="int">2880</bufsize>
            <samp sid="samp" type="int">3</samp>
            <buffers sid="buffers" type="int">1</buffers>
            <area_shape sid="area_shape" type="int">1</area_shape>
            <area_size sid="area_size" type="float">0.1</area_size>
            <area_sizey sid="area_sizey" type="float">0.1</area_sizey>
            <area_sizez sid="area_sizez" type="float">1</area_sizez>
            </technique>
        </extra>
        </light>
    </library_lights>
    <library_effects>
        <effect id="Material-effect">
        <profile_COMMON>
            <technique sid="common">
            <lambert>
                <emission>
                <color sid="emission">0 0 0 1</color>
                </emission>
                <diffuse>
                <color sid="diffuse">0.8 0.8 0.8 1</color>
                </diffuse>
                <index_of_refraction>
                <float sid="ior">1.45</float>
                </index_of_refraction>
            </lambert>
            </technique>
        </profile_COMMON>
        </effect>
    </library_effects>
    <library_images/>
    <library_materials>
        <material id="Material-material" name="Material">
        <instance_effect url="#Material-effect"/>
        </material>
    </library_materials>
    <library_geometries>
        <geometry id="Cube-mesh" name="Cube">
        <mesh>
            <source id="Cube-mesh-positions">
            <float_array id="Cube-mesh-positions-array" count="24">1 1 1 1 1 -1 1 -1 1 1 -1 -1 -1 1 1 -1 1 -1 -1 -1 1 -1 -1 -1</float_array>
            <technique_common>
                <accessor source="#Cube-mesh-positions-array" count="8" stride="3">
                <param name="X" type="float"/>
                <param name="Y" type="float"/>
                <param name="Z" type="float"/>
                </accessor>
            </technique_common>
            </source>
            <source id="Cube-mesh-normals">
            <float_array id="Cube-mesh-normals-array" count="18">0 0 1 0 -1 0 -1 0 0 0 0 -1 1 0 0 0 1 0</float_array>
            <technique_common>
                <accessor source="#Cube-mesh-normals-array" count="6" stride="3">
                <param name="X" type="float"/>
                <param name="Y" type="float"/>
                <param name="Z" type="float"/>
                </accessor>
            </technique_common>
            </source>
            <source id="Cube-mesh-map-0">
            <float_array id="Cube-mesh-map-0-array" count="72">0.625 0 0.375 0.25 0.375 0 0.625 0.25 0.375 0.5 0.375 0.25 0.625 0.5 0.375 0.75 0.375 0.5 0.625 0.75 0.375 1 0.375 0.75 0.375 0.5 0.125 0.75 0.125 0.5 0.875 0.5 0.625 0.75 0.625 0.5 0.625 0 0.625 0.25 0.375 0.25 0.625 0.25 0.625 0.5 0.375 0.5 0.625 0.5 0.625 0.75 0.375 0.75 0.625 0.75 0.625 1 0.375 1 0.375 0.5 0.375 0.75 0.125 0.75 0.875 0.5 0.875 0.75 0.625 0.75</float_array>
            <technique_common>
                <accessor source="#Cube-mesh-map-0-array" count="36" stride="2">
                <param name="S" type="float"/>
                <param name="T" type="float"/>
                </accessor>
            </technique_common>
            </source>
            <vertices id="Cube-mesh-vertices">
            <input semantic="POSITION" source="#Cube-mesh-positions"/>
            </vertices>
            <triangles material="Material-material" count="12">
            <input semantic="VERTEX" source="#Cube-mesh-vertices" offset="0"/>
            <input semantic="NORMAL" source="#Cube-mesh-normals" offset="1"/>
            <input semantic="TEXCOORD" source="#Cube-mesh-map-0" offset="2" set="0"/>
            <p>4 0 0 2 0 1 0 0 2 2 1 3 7 1 4 3 1 5 6 2 6 5 2 7 7 2 8 1 3 9 7 3 10 5 3 11 0 4 12 3 4 13 1 4 14 4 5 15 1 5 16 5 5 17 4 0 18 6 0 19 2 0 20 2 1 21 6 1 22 7 1 23 6 2 24 4 2 25 5 2 26 1 3 27 3 3 28 7 3 29 0 4 30 2 4 31 3 4 32 4 5 33 0 5 34 1 5 35</p>
            </triangles>
        </mesh>
        </geometry>
    </library_geometries>
    <library_visual_scenes>
        <visual_scene id="Scene" name="Scene">
        <node id="Camera" name="Camera" type="NODE">
            <matrix sid="transform">0.6859207 -0.3240135 0.6515582 7.358891 0.7276763 0.3054208 -0.6141704 -6.925791 0 0.8953956 0.4452714 4.958309 0 0 0 1</matrix>
            <instance_camera url="#Camera-camera"/>
        </node>
        <node id="Light" name="Light" type="NODE">
            <matrix sid="transform">-0.2908646 -0.7711008 0.5663932 4.076245 0.9551712 -0.1998834 0.2183912 1.005454 -0.05518906 0.6045247 0.7946723 5.903862 0 0 0 1</matrix>
            <instance_light url="#Light-light"/>
        </node>
        <node id="Cube" name="Cube" type="NODE">
            <matrix sid="transform">1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1</matrix>
            <instance_geometry url="#Cube-mesh" name="Cube">
            <bind_material>
                <technique_common>
                <instance_material symbol="Material-material" target="#Material-material">
                    <bind_vertex_input semantic="UVMap" input_semantic="TEXCOORD" input_set="0"/>
                </instance_material>
                </technique_common>
            </bind_material>
            </instance_geometry>
        </node>
        </visual_scene>
    </library_visual_scenes>
    <scene>
        <instance_visual_scene url="#Scene"/>
    </scene>
    </COLLADA>"##;
