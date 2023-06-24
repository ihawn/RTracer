use crate::spacial::tri::Tri;
use crate::spacial::mesh_object::MeshObject;
use crate::datatypes::vector3::Vector3;
use std::cmp::Ordering;
use rand::Rng;
use std::time::Instant;

pub struct BVH {
    pub bvh_obj_1: Option<Box<BVH>>,
    pub bvh_obj_2: Option<Box<BVH>>,
    pub tri: Tri,
    pub bb_corner_1: Vector3,
    pub bb_corner_2: Vector3,
    pub is_leaf: bool,
}

impl BVH {
    pub fn new(mesh_objects: &[MeshObject]) -> BVH {
        let mut tris: Vec<Tri> = vec![];
        for mesh in mesh_objects {
            for m in &mesh.tris {
                tris.push(*m);
            }
        }
        let start_time = Instant::now();
        println!("Building BVH");
        let len: usize = tris.len();
        let bvh: BVH = Self::construct_recursive(&mut tris, 0, len);
        let elapsed_time = start_time.elapsed().as_millis();
        println!("Done");
        println!("Built BVH in {} seconds", elapsed_time as f64 / 1000.0);

        bvh
    }

    fn construct_recursive(tris: &mut [Tri], start: usize, end: usize) -> BVH {
        let object_span = end - start;
        if object_span == 1 {
            let bb = tris[start].bounding_box;
            return BVH {
                bvh_obj_1: None,
                bvh_obj_2: None,
                tri: tris[start].clone(),
                bb_corner_1: bb.0,
                bb_corner_2: bb.1,
                is_leaf: true,
            };
        } else if object_span == 2 {
            if Self::box_compare(&tris[start], &tris[start + 1], 0) == Ordering::Less {
                let mid = start + 1;
                let (tris1, tris2) = tris.split_at_mut(mid);
                let (bv1, bv2) = rayon::join(
                    || Self::construct_recursive(tris1, start, mid),
                    || Self::construct_recursive(tris2, 0, end - mid),
                );

                let bounding_box = Self::merge_bounding_boxes(
                    (bv1.bb_corner_1, bv1.bb_corner_2),
                    (bv2.bb_corner_1, bv2.bb_corner_2),
                );
                return BVH {
                    bvh_obj_1: Some(Box::new(bv1)),
                    bvh_obj_2: Some(Box::new(bv2)),
                    tri: Tri::empty(),
                    bb_corner_1: bounding_box.0,
                    bb_corner_2: bounding_box.1,
                    is_leaf: false,
                };
            } else {
                let mid = start + 1;
                let (tris1, tris2) = tris.split_at_mut(mid);
                let (bv1, bv2) = rayon::join(
                    || Self::construct_recursive(tris2, 0, end - mid),
                    || Self::construct_recursive(tris1, start, mid),
                );
                let bounding_box = Self::merge_bounding_boxes(
                    (bv1.bb_corner_1, bv1.bb_corner_2),
                    (bv2.bb_corner_1, bv2.bb_corner_2),
                );
                return BVH {
                    bvh_obj_1: Some(Box::new(bv1)),
                    bvh_obj_2: Some(Box::new(bv2)),
                    tri: Tri::empty(),
                    bb_corner_1: bounding_box.0,
                    bb_corner_2: bounding_box.1,
                    is_leaf: false,
                };
            }
        }

        let axis: usize = rand::thread_rng().gen_range(0..=2);
        tris[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));

        let mid: usize = start + object_span / 2;
        let (tris1, tris2) = tris.split_at_mut(mid);
        let (bvh_obj_1, bvh_obj_2) = rayon::join(
            || Self::construct_recursive(tris1, start, mid),
            || Self::construct_recursive(tris2, 0, end - mid),
        );
        let bounding_box = Self::merge_bounding_boxes(
            (bvh_obj_1.bb_corner_1, bvh_obj_1.bb_corner_2),
            (bvh_obj_2.bb_corner_1, bvh_obj_2.bb_corner_2),
        );

        BVH {
            bvh_obj_1: Some(Box::new(bvh_obj_1)),
            bvh_obj_2: Some(Box::new(bvh_obj_2)),
            tri: Tri::empty(),
            bb_corner_1: bounding_box.0,
            bb_corner_2: bounding_box.1,
            is_leaf: false,
        }
    }

    fn box_compare(a: &Tri, b: &Tri, axis: usize) -> Ordering {
        match axis {
            0 => a.bounding_box_center.x.partial_cmp(&b.bounding_box_center.x).unwrap_or(Ordering::Equal),
            1 => a.bounding_box_center.y.partial_cmp(&b.bounding_box_center.y).unwrap_or(Ordering::Equal),
            2 => a.bounding_box_center.z.partial_cmp(&b.bounding_box_center.z).unwrap_or(Ordering::Equal),
            _ => panic!("Invalid axis value"),
        }
    }

    pub fn merge_bounding_boxes(box1: (Vector3, Vector3), box2: (Vector3, Vector3)) -> (Vector3, Vector3) {
        (
            box1.0.min(box1.1.min(box2.0.min(box2.1))),
            box1.0.max(box1.1.max(box2.0.max(box2.1))),
        )
    }
}
