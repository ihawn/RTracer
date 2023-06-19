use crate::spacial::mesh::Mesh;

#[derive(Clone)]
pub struct MeshObject {
    pub tris: Vec<Mesh>,
    pub smooth_shading: bool
}

impl MeshObject {
    pub fn new(mut tris: Vec<Mesh>, smooth_shading: bool) -> MeshObject {
        for i in 0..tris.len() {
            tris[i].smooth_shading = smooth_shading;
        }
        MeshObject { tris: tris, smooth_shading }
    }
}