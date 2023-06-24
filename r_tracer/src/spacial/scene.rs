use crate::spacial::tri::Tri;
use crate::spacial::mesh_object::MeshObject;
use crate::datatypes::color::Color;


#[derive(Clone)]
pub struct Scene {
    pub meshes: Vec<MeshObject>,
    pub env_color: Color,
}

impl Scene {
    pub fn new(meshes: Vec<MeshObject>, env_color: Color) -> Scene {
        Scene {
            meshes: meshes,
            env_color: env_color,
        }
    }
}