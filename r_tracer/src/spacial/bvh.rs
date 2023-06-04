use crate::spacial::mesh::Mesh;
use crate::datatypes::vector3::Vector3;

pub struct BVH {
    pub bvh_obj_1: Option<Box<BVH>>,
    pub bvh_obj_2: Option<Box<BVH>>,
    pub mesh_1: Mesh,
    pub mesh_2: Mesh,
    pub bb_corner_1: Vector3,
    pub bb_corner_2: Vector3,
    pub is_leaf: bool
}

impl BVH {
    pub fn new_bvh_branch(bvh_1: BVH, bvh_2: BVH) -> BVH {
        let b1: BVH = bvh_1.into();
        let b2: BVH = bvh_2.into();
        let bounding_box = Self::merge_bounding_boxes(
            (b1.bb_corner_1, b1.bb_corner_2), (b2.bb_corner_1, b2.bb_corner_2)
        );
        BVH {
            bvh_obj_1: Some(Box::new(b1)),
            bvh_obj_2: Some(Box::new(b2)),
            mesh_1: Mesh::empty(),
            mesh_2: Mesh::empty(),
            bb_corner_1: bounding_box.0,
            bb_corner_2: bounding_box.1,
            is_leaf: false
        }
    }

    pub fn new_bvh_leaf(mesh_1: Mesh, mesh_2: Mesh) -> BVH {
        let bb = Self::merge_bounding_boxes(mesh_1.bounding_box, mesh_2.bounding_box);
        BVH {
            bvh_obj_1: None,
            bvh_obj_2: None,
            mesh_1: mesh_1,
            mesh_2: mesh_2,
            bb_corner_1: bb.0,
            bb_corner_2: bb.1,
            is_leaf: true
        }
    }

    pub fn merge_bounding_boxes(box1: (Vector3, Vector3), box2: (Vector3, Vector3)) -> (Vector3, Vector3) {
        (box1.0.min(box1.1.min(box2.0.min(box2.1))), box1.0.max(box1.1.max(box2.0.max(box2.1))))
    }
}