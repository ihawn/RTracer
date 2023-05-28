use crate::spacial::sphere::Sphere;

#[derive(Copy, Clone)]
pub struct Scene {
    pub sphere: Sphere
}

impl Scene {
    pub fn new(sphere: Sphere) -> Scene {
        Scene { sphere: sphere }
    }
}