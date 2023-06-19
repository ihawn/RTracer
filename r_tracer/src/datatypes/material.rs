use crate::datatypes::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub emission_color: Color,
    pub specular_color: Color,
    pub emission_strength: f64,
    pub smoothness: f64,
    pub specular: f64,
    pub dielectric: f64,
    pub index_of_refraction: f64,
    pub visible: bool
}

impl Material {
    pub fn new(color: Color, emiss_color: Color, spec_color: Color, 
        emiss_strength: f64, smoothness: f64, specular: f64,
        dielectric: f64, ior: f64, visible: bool) -> Material {
        Material {
            color: color,
            emission_color: emiss_color,
            specular_color: spec_color,
            emission_strength: emiss_strength,
            smoothness: smoothness,
            specular: specular,
            dielectric: dielectric,
            index_of_refraction: ior,
            visible: visible
        }
    }

    pub fn empty() -> Material {
        Material {
            color: Color::black(),
            emission_color: Color::black(),
            specular_color: Color::black(),
            emission_strength: 0.0,
            smoothness: 0.0,
            specular: 0.0,
            dielectric: 0.0,
            index_of_refraction: 0.0,
            visible: false
        }
    }
}