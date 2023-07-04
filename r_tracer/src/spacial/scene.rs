use crate::spacial::mesh_object::MeshObject;
use crate::datatypes::color::Color;
use crate::datatypes::vector2d::Vector2D;


#[derive(Clone)]
pub struct Scene {
    pub meshes: Vec<MeshObject>,
    pub texture_maps: Vec<Vector2D<Color>>,
    pub env_color: Color
}

impl Scene {
    pub fn new(meshes: Vec<MeshObject>, albedo_maps: Vec<Vector2D<Color>>, env_color: Color) -> Scene {
        Scene {
            meshes: meshes,
            texture_maps: albedo_maps,
            env_color: env_color
        }
    }
}