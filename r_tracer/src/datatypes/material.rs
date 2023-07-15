use crate::datatypes::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Color,
    pub emission_color: Color,
    pub specular_color: Color,
    pub dielectric_color: Color,
    pub emission_strength: f32,
    pub smoothness: f32,
    pub specular: f32,
    pub dielectric: f32,
    pub index_of_refraction: f32,
    pub normal_strength: f32,
    pub visible: bool,
    pub diffuse_color_map_index: Option<usize>,
    pub emission_color_map_index: Option<usize>,
    pub specular_color_map_index: Option<usize>,
    pub dielectric_color_map_index: Option<usize>,
    pub normal_map_index: Option<usize>,
    pub smoothness_map_index: Option<usize>,
    pub specular_map_index: Option<usize>,
}

impl Material {
    pub fn new(color: Color, emiss_color: Color, spec_color: Color, dielectric_color: Color,
        emiss_strength: f32, smoothness: f32, specular: f32,
        dielectric: f32, ior: f32, normal_strength: f32, visible: bool, diffuse_color_map_id: Option<usize>,
        emission_color_map_id: Option<usize>, specular_color_map_id: Option<usize>,
        dielectric_color_map_id: Option<usize>, normal_map_id: Option<usize>, 
        smoothness_map_id: Option<usize>, specular_map_id: Option<usize>) -> Material {
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
            normal_strength: normal_strength,
            visible: visible,
            diffuse_color_map_index: diffuse_color_map_id,
            emission_color_map_index: emission_color_map_id,
            specular_color_map_index: specular_color_map_id,
            dielectric_color_map_index: dielectric_color_map_id,
            normal_map_index: normal_map_id,
            smoothness_map_index: smoothness_map_id,
            specular_map_index: specular_map_id
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
            normal_strength: 0.0,
            visible: false,
            diffuse_color_map_index: None,
            emission_color_map_index: None,
            specular_color_map_index: None,
            dielectric_color_map_index: None,
            normal_map_index: None,
            smoothness_map_index: None,
            specular_map_index: None
        }
    }
}