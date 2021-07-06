use crate::math::{Color, Vector};

use super::{Material, ScatterRecord};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _hit_direction: Vector, hit_normal: Vector) -> ScatterRecord {
        let non_unit_scatter_direction = hit_normal + Vector::random_unit();

        if Vector::dot(&non_unit_scatter_direction, &hit_normal) > 0.0 {
            // ray was scattered
            ScatterRecord::NonIdeal {
                attenuation: self.albedo,
                direction: non_unit_scatter_direction.unit(),
            }
        } else {
            // ray was absorbed
            ScatterRecord::Ideal {
                color: Color::new(0.0, 0.0, 0.0),
            }
        }
    }
}
