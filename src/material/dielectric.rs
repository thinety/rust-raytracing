use crate::math::{Color, Vector};

use super::{refract, Material, ScatterRecord};

pub struct Dielectric {
    pub albedo: Color,
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, hit_direction: Vector, hit_normal: Vector) -> ScatterRecord {
        let refract_direction = refract(hit_direction, hit_normal, self.refraction_index);

        ScatterRecord::NonIdeal {
            attenuation: self.albedo,
            direction: refract_direction,
        }
    }
}
