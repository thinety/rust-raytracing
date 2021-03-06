use crate::math::{Color, Vector};

use super::{MaterialInterface, ScatterRecord};

pub struct BlackBody {
    pub color: Color,
}

impl MaterialInterface for BlackBody {
    fn scatter(&self, _hit_direction: Vector, _hit_normal: Vector) -> ScatterRecord {
        ScatterRecord::Ideal { color: self.color }
    }
}

pub struct BlackBodyNormal {}

impl MaterialInterface for BlackBodyNormal {
    fn scatter(&self, _hit_direction: Vector, hit_normal: Vector) -> ScatterRecord {
        let normal_color = Color::new(hit_normal.x, hit_normal.y, hit_normal.z);
        let white = Color::new(1.0, 1.0, 1.0);
        ScatterRecord::Ideal {
            color: 0.5 * (normal_color + white),
        }
    }
}
