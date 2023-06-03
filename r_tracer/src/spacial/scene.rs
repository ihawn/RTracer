use crate::spacial::mesh::Mesh;
use crate::datatypes::color::Color;

#[derive(Clone)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub env_color: Color,
}

impl Scene {
    pub fn new(meshes: Vec<Mesh>, env_color: Color) -> Scene {
        Scene {
            meshes: meshes,
            env_color,
        }
    }
}