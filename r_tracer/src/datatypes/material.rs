use crate::datatypes::color::Color;
use image::{Rgb, RgbImage};


#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Color,
    pub emission_color: Color,
    pub specular_color: Color,
    pub dielectric_color: Color,
    pub emission_strength: f64,
    pub smoothness: f64,
    pub specular: f64,
    pub dielectric: f64,
    pub index_of_refraction: f64,
    pub visible: bool,
    pub diffuse_color_map_index: Option<usize>,
    pub emission_color_map_index: Option<usize>,
    pub specular_color_map_index: Option<usize>,
    pub dielectric_color_map_index: Option<usize>
}

impl Material {
    pub fn new(color: Color, emiss_color: Color, spec_color: Color, dielectric_color: Color,
        emiss_strength: f64, smoothness: f64, specular: f64,
        dielectric: f64, ior: f64, visible: bool, color_map_id: Option<usize>,
        emission_map_id: Option<usize>, specular_map_id: Option<usize>,
        dielectric_color_map_id: Option<usize>) -> Material {
        Material {
            diffuse_color: color,
            emission_color: emiss_color,
            specular_color: spec_color,
            dielectric_color: dielectric_color,
            emission_strength: emiss_strength,
            smoothness: smoothness,
            specular: specular,
            dielectric: dielectric,
            index_of_refraction: ior,
            visible: visible,
            diffuse_color_map_index: color_map_id,
            emission_color_map_index: emission_map_id,
            specular_color_map_index: specular_map_id,
            dielectric_color_map_index: dielectric_color_map_id
        }
    }

    pub fn empty() -> Material {
        Material {
            diffuse_color: Color::black(),
            emission_color: Color::black(),
            specular_color: Color::black(),
            dielectric_color: Color::black(),
            emission_strength: 0.0,
            smoothness: 0.0,
            specular: 0.0,
            dielectric: 0.0,
            index_of_refraction: 0.0,
            visible: false,
            diffuse_color_map_index: None,
            emission_color_map_index: None,
            specular_color_map_index: None,
            dielectric_color_map_index: None
        }
    }
}