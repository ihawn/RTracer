use crate::spacial::sphere::Sphere;

#[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>) -> Scene {
        Scene { 
            spheres: spheres
        }
    }
}
