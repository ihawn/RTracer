use crate::spacial::mesh::Mesh;
use crate::datatypes::vector3::Vector3;
use std::cmp::Ordering;

use super::mesh::PrimitiveMeshType;

pub struct BVH {
    pub bvh_obj_1: Option<Box<BVH>>,
    pub bvh_obj_2: Option<Box<BVH>>,
    pub mesh: Mesh,
    pub bb_corner_1: Vector3,
    pub bb_corner_2: Vector3,
    pub is_leaf: bool
}

impl BVH {
    pub fn new(meshes: &Vec<Mesh>) -> BVH {
        let mut tris: Vec<Mesh> = vec![];
        for m in meshes {
            if m.mesh_type == PrimitiveMeshType::Triangle { tris.push(*m) }
        }
        Self::construct_recursive(&tris)
    }

    fn construct_recursive(meshes: &Vec<Mesh>) -> BVH {
        let num_meshes = meshes.len();
        if num_meshes == 1 {
            let mesh = meshes.into_iter().next().unwrap();
            let bb = mesh.bounding_box;
            BVH {
                bvh_obj_1: None,
                bvh_obj_2: None,
                mesh: *mesh,
                bb_corner_1: bb.0,
                bb_corner_2: bb.1,
                is_leaf: true,
            }
        } else {
            let mut sorted_meshes = meshes.clone();
            sorted_meshes.sort_by(|a, b| {
                let a_center = a.bounding_box_center;
                let b_center = b.bounding_box_center;
            
                match a_center.x.partial_cmp(&b_center.x) {
                    Some(Ordering::Equal) => {
                        match a_center.y.partial_cmp(&b_center.y) {
                            Some(Ordering::Equal) => {
                                a_center.z.partial_cmp(&b_center.z).unwrap_or(Ordering::Equal)
                            }
                            other => other.unwrap(),
                        }
                    }
                    other => other.unwrap(),
                }
            });

            let mid = num_meshes / 2;
            let (meshes_1, meshes_2) = sorted_meshes.split_at(mid);

            let m1 = meshes_1.to_vec();
            let m2 = meshes_2.to_vec();
            let bvh_obj_1 = Self::construct_recursive(&m1);
            let bvh_obj_2 = Self::construct_recursive(&m2);
            let bounding_box = Self::merge_bounding_boxes(
                (bvh_obj_1.bb_corner_1, bvh_obj_1.bb_corner_2),
                (bvh_obj_2.bb_corner_1, bvh_obj_2.bb_corner_2),
            );

            BVH {
                bvh_obj_1: Some(Box::new(bvh_obj_1)),
                bvh_obj_2: Some(Box::new(bvh_obj_2)),
                mesh: Mesh::empty(),
                bb_corner_1: bounding_box.0,
                bb_corner_2: bounding_box.1,
                is_leaf: false,
            }
        }
    }

    pub fn merge_bounding_boxes(box1: (Vector3, Vector3), box2: (Vector3, Vector3)) -> (Vector3, Vector3) {
        (box1.0.min(box1.1.min(box2.0.min(box2.1))), box1.0.max(box1.1.max(box2.0.max(box2.1))))
    }
}