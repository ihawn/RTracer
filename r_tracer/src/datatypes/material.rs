use crate::datatypes::vector3::Vector3;
use crate::datatypes::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub emission_color: Color,
    pub emission_strength: f64
}

impl Material {
    pub fn new(color: Color, emiss_color: Color, emiss_strength: f64) -> Material {
        Material {
            color: color,
            emission_color: emiss_color,
            emission_strength: emiss_strength
        }
    }
}