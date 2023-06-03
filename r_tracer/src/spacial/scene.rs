use crate::spacial::sphere::Sphere;
use crate::datatypes::color::Color;

#[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub env_color: Color
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, env_color: Color) -> Scene {
        Scene { 
            spheres: spheres,
            env_color: env_color
        }
    }
}
