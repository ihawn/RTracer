
pub mod utilities {
    pub mod frame_handler;
    pub mod file_utilities;
    pub mod postprocessing;
}

pub mod datatypes {
    pub mod color;
    pub mod vector2d;
    pub mod vector3d;
    pub mod vector2;
    pub mod vector3;
    pub mod hit_point;
    pub mod material;
}

pub mod spacial {
    pub mod camera;
    pub mod scene;
    pub mod tri;  
    pub mod mesh_object;
    pub mod ray;
    pub mod bvh;
}
