mod black_body;
mod dielectric;
mod lambertian;
mod metal;

use enum_dispatch::enum_dispatch;

use crate::math::{Color, Vector};

pub use black_body::{BlackBody, BlackBodyNormal};
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

#[enum_dispatch]
pub enum Material {
    BlackBody,
    BlackBodyNormal,
    Lambertian,
    Metal,
    Dielectric,
}

#[enum_dispatch(Material)]
pub trait MaterialInterface {
    fn scatter(&self, hit_direction: Vector, hit_normal: Vector) -> ScatterRecord;
}

pub enum ScatterRecord {
    Ideal {
        color: Color,
    },
    NonIdeal {
        attenuation: Color,
        direction: Vector,
    },
}
