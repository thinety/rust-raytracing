pub mod black_body;
mod lambertian;
mod metal;

use crate::math::{Color, Vector};

pub use lambertian::Lambertian;
pub use metal::Metal;

pub enum ScatterRecord {
    Ideal {
        color: Color,
    },
    NonIdeal {
        attenuation: Color,
        direction: Vector,
    },
}

pub trait Material {
    fn scatter(&self, hit_direction: Vector, hit_normal: Vector) -> ScatterRecord;
}
