use crate::datatypes::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub emission_color: Color,
    pub specular_color: Color,
    pub emission_strength: f64,
    pub smoothness: f64,
    pub specular: f64
}

impl Material {
    pub fn new(color: Color, emiss_color: Color, spec_color: Color, 
        emiss_strength: f64, smoothness: f64, specular: f64) -> Material {
        Material {
            color: color,
            emission_color: emiss_color,
            specular_color: spec_color,
            emission_strength: emiss_strength,
            smoothness: smoothness,
            specular: specular
        }
    }

    pub fn empty() -> Material {
        Material {
            color: Color::black(),
            emission_color: Color::black(),
            specular_color: Color::black(),
            emission_strength: 0.0,
            smoothness: 0.0,
            specular: 0.0
        }
    }
}