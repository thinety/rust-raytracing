mod black_body;
mod dielectric;
mod lambertian;
mod metal;

use crate::math::{Color, Vector};

pub use black_body::{BlackBody, BlackBodyNormal};
pub use dielectric::Dielectric;
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

impl<T> Material for T
where
    T: std::ops::Deref,
    <T as std::ops::Deref>::Target: Material,
{
    fn scatter(&self, hit_direction: Vector, hit_normal: Vector) -> ScatterRecord {
        (**self).scatter(hit_direction, hit_normal)
    }
}
