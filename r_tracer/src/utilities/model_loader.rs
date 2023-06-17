use pk_stl::parse_stl;
use crate::spacial::mesh::Mesh;
use crate::datatypes::material::Material;
use crate::datatypes::vector3::Vector3;
use std::fs;

pub fn load_model(file_path: &str, material: Material) -> Vec<Mesh>  {
    let content = fs::read(file_path).expect("Failed to read model file");
    let model = parse_stl(content.as_slice()).unwrap();

    let mut model_tris: Vec<Mesh> = vec![];
    let mut outer_counter = 0;
    for tri in &model.triangles {

        let vertex1: Vector3 = Vector3::new(tri.vertices[0].x.into(), tri.vertices[0].y.into(), tri.vertices[0].z.into());
        let vertex2: Vector3 = Vector3::new(tri.vertices[1].x.into(), tri.vertices[1].y.into(), tri.vertices[1].z.into());
        let vertex3: Vector3 = Vector3::new(tri.vertices[2].x.into(), tri.vertices[2].y.into(), tri.vertices[2].z.into());
        let face_normal = Vector3::new(tri.normal.x.into(), tri.normal.y.into(), tri.normal.z.into());

        let mut vertex1_normal: Vector3 = Vector3::zero();
        let mut vertex2_normal: Vector3 = Vector3::zero();
        let mut vertex3_normal: Vector3 = Vector3::zero();

        let e: f64 = 0.0001;
        for other_tri in &model.triangles {
            let other_vertex1: Vector3 = Vector3::new(other_tri.vertices[0].x.into(), other_tri.vertices[0].y.into(), other_tri.vertices[0].z.into());
            let other_vertex2: Vector3 = Vector3::new(other_tri.vertices[1].x.into(), other_tri.vertices[1].y.into(), other_tri.vertices[1].z.into());
            let other_vertex3: Vector3 = Vector3::new(other_tri.vertices[2].x.into(), other_tri.vertices[2].y.into(), other_tri.vertices[2].z.into());
            let other_face_normal = Vector3::new(other_tri.normal.x.into(), other_tri.normal.y.into(), other_tri.normal.z.into());

            if (vertex1 - other_vertex1).magnitude() < e
                || (vertex1 - other_vertex2).magnitude() < e
                || (vertex1 - other_vertex3).magnitude() < e
            {
                vertex1_normal += other_face_normal;
            }
            if (vertex2 - other_vertex1).magnitude() < e
                || (vertex2 - other_vertex2).magnitude() < e
                || (vertex2 - other_vertex3).magnitude() < e
            {
                vertex2_normal += other_face_normal;
            }
            if (vertex3 - other_vertex1).magnitude() < e
                || (vertex3 - other_vertex2).magnitude() < e
                || (vertex3 - other_vertex3).magnitude() < e
            {
                vertex3_normal += other_face_normal;
            }
        }

        vertex1_normal = vertex1_normal.normalize();
        vertex2_normal = vertex2_normal.normalize();
        vertex3_normal = vertex3_normal.normalize();

        let tri: Mesh = Mesh::new_triangle(
            vertex1, vertex2, vertex3, 
            vertex1_normal, vertex2_normal, vertex3_normal,
            face_normal, material
        );
        model_tris.push(tri);
        outer_counter += 1;
        if outer_counter % 1000 == 0 || outer_counter == model.triangles.len()
        {
             println!("Computing vertex normals {}/{}", outer_counter, model.triangles.len()); 
        }
    }

    model_tris
}

pub fn compute_area(p1: Vector3, p2: Vector3, p3: Vector3) -> f64 {
    let v0 = p2 - p1;
    let v1 = p3 - p1;
    let cross_product = v0.cross(&v1);
    cross_product.magnitude() * 0.5
}