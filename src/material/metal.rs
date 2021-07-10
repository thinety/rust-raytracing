use crate::math::{Color, Vector};

use super::{MaterialInterface, ScatterRecord};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl MaterialInterface for Metal {
    fn scatter(&self, hit_direction: Vector, hit_normal: Vector) -> ScatterRecord {
        let reflect_direction = reflect(hit_direction, hit_normal);
        let non_unit_scatter_direction = reflect_direction + self.fuzz * Vector::random_unit();

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

pub fn reflect(hit_direction: Vector, hit_normal: Vector) -> Vector {
    let reflect_direction =
        hit_direction - 2.0 * Vector::dot(&hit_direction, &hit_normal) * hit_normal;
    reflect_direction
}
