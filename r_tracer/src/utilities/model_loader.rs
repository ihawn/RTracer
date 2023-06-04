use pk_stl::parse_stl;
use crate::spacial::mesh::Mesh;
use crate::datatypes::material::Material;
use crate::datatypes::vector3::Vector3;
use std::fs;

pub fn load_model(file_path: &str, material: Material) -> Vec<Mesh>  {
    let content = fs::read(file_path).expect("Failed to read model file");
    let model = parse_stl(content.as_slice()).unwrap();

    let mut model_tris: Vec<Mesh> = vec![];
    for tri in model.triangles {
        model_tris.push(
            Mesh::new_triangle(
                Vector3::new(tri.vertices[0].x.into(), tri.vertices[0].y.into(), tri.vertices[0].z.into()),
                Vector3::new(tri.vertices[1].x.into(), tri.vertices[1].y.into(), tri.vertices[1].z.into()),
                Vector3::new(tri.vertices[2].x.into(), tri.vertices[2].y.into(), tri.vertices[2].z.into()),
                Vector3::new(tri.normal.x.into(), tri.normal.y.into(), tri.normal.z.into()), material)
        );
    }

    model_tris
}

