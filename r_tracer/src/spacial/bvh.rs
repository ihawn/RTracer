use crate::spacial::mesh::Mesh;
use crate::datatypes::vector3::Vector3;
use std::cmp::Ordering;
use rand::Rng;

use super::mesh::{PrimitiveMeshType, self};

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
        Self::construct_recursive(&tris, 0, tris.len())
    }

    fn construct_recursive(meshes: &Vec<Mesh>, start: usize, end: usize) -> BVH {
        let object_span = end - start;
        if object_span == 1 {
            let bb = meshes[start].bounding_box;
            return BVH {
                bvh_obj_1: None,
                bvh_obj_2: None,
                mesh: meshes[start].clone(),
                bb_corner_1: bb.0,
                bb_corner_2: bb.1,
                is_leaf: true,
            };
        } else if object_span == 2 {
            if Self::box_compare(&meshes[start], &meshes[start + 1], 0) == Ordering::Less {
                let bv1 = Self::construct_recursive(meshes, start, start + 1);
                let bv2 = Self::construct_recursive(meshes, start + 1, end);
                let bounding_box = Self::merge_bounding_boxes(
                    (bv1.bb_corner_1, bv1.bb_corner_2),
                    (bv2.bb_corner_1, bv2.bb_corner_2),
                );
                return BVH {
                    bvh_obj_1: Some(Box::new(bv1)),
                    bvh_obj_2: Some(Box::new(bv2)),
                    mesh: Mesh::empty(),
                    bb_corner_1: bounding_box.0,
                    bb_corner_2: bounding_box.1,
                    is_leaf: false,
                };
            } else {
                let bv1 = Self::construct_recursive(meshes, start + 1, end);
                let bv2 = Self::construct_recursive(meshes, start, start + 1);
                let bounding_box = Self::merge_bounding_boxes(
                    (bv1.bb_corner_1, bv1.bb_corner_2),
                    (bv2.bb_corner_1, bv2.bb_corner_2),
                );
                return BVH {
                    bvh_obj_1: Some(Box::new(bv1)),
                    bvh_obj_2: Some(Box::new(bv2)),
                    mesh: Mesh::empty(),
                    bb_corner_1: bounding_box.0,
                    bb_corner_2: bounding_box.1,
                    is_leaf: false,
                };
            }
        }
    
        let axis = rand::thread_rng().gen_range(0..=2);
        let mut sub_meshes = meshes.clone();
        sub_meshes[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));
    
        let mid = start + object_span / 2;
        let bvh_obj_1 = Self::construct_recursive(&sub_meshes, start, mid);
        let bvh_obj_2 = Self::construct_recursive(&sub_meshes, mid, end);
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
    

    fn box_compare(a: &Mesh, b: &Mesh, axis: usize) -> Ordering {
        match axis {
            0 => a.bounding_box_center.x.partial_cmp(&b.bounding_box_center.x).unwrap_or(Ordering::Equal),
            1 => a.bounding_box_center.y.partial_cmp(&b.bounding_box_center.y).unwrap_or(Ordering::Equal),
            2 => a.bounding_box_center.z.partial_cmp(&b.bounding_box_center.z).unwrap_or(Ordering::Equal),
            _ => panic!("Invalid axis value"),
        }
    }

    pub fn merge_bounding_boxes(box1: (Vector3, Vector3), box2: (Vector3, Vector3)) -> (Vector3, Vector3) {
        (box1.0.min(box1.1.min(box2.0.min(box2.1))), box1.0.max(box1.1.max(box2.0.max(box2.1))))
    }
}