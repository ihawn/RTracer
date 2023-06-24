use crate::spacial::tri::Tri;

#[derive(Clone)]
pub struct MeshObject {
    pub tris: Vec<Tri>,
    pub smooth_shading: bool
}

impl MeshObject {
    pub fn new(mut tris: Vec<Tri>, smooth_shading: bool) -> MeshObject {
        for i in 0..tris.len() {
            tris[i].smooth_shading = smooth_shading;
        }
        MeshObject { tris: tris, smooth_shading }
    }
}