use crate::spacial::mesh::Mesh;
use crate::spacial::mesh_object::MeshObject;
use crate::datatypes::color::Color;


#[derive(Clone)]
pub struct Scene {
    pub meshes: Vec<MeshObject>,
    pub spheres: Vec<Mesh>,
    pub env_color: Color,
}

impl Scene {
    pub fn new(meshes: Vec<MeshObject>, spheres: Vec<Mesh>, env_color: Color) -> Scene {
        Scene {
            meshes: meshes,
            spheres: spheres,
            env_color: env_color,
        }
    }
}