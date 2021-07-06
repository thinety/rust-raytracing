pub mod black_body;
mod dielectric;
mod lambertian;
mod metal;

use rand::Rng;

use crate::math::{Color, Vector};

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

fn reflect(hit_direction: Vector, hit_normal: Vector) -> Vector {
    let reflect_direction =
        hit_direction - 2.0 * Vector::dot(&hit_direction, &hit_normal) * hit_normal;
    reflect_direction
}

fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    // Schlick's approximation for reflectance
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    let r_theta = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
    r_theta
}

fn refract(hit_direction: Vector, hit_normal: Vector, refraction_index: f64) -> Vector {
    let refract_normal;
    let refraction_ratio;

    // this dot product can never be 0.0 because a hit is only
    // recorded if it is not ortogonal to the normal vector
    if Vector::dot(&hit_direction, &hit_normal) < 0.0 {
        // ray entering material
        refract_normal = -hit_normal;
        refraction_ratio = 1.0 / refraction_index;
    } else {
        // ray leaving material
        refract_normal = hit_normal;
        refraction_ratio = refraction_index / 1.0;
    }

    let cos_theta = Vector::dot(&hit_direction, &refract_normal);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let refract_direction;
    if refraction_ratio * sin_theta > 1.0
        || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0)
    {
        // reflects
        refract_direction = reflect(hit_direction, -refract_normal);
    } else {
        // refracts
        let refract_perpendicular = refraction_ratio * (hit_direction - cos_theta * refract_normal);
        let refract_parallel = (1.0 - Vector::dot(&refract_perpendicular, &refract_perpendicular))
            .sqrt()
            * refract_normal;
        refract_direction = refract_perpendicular + refract_parallel;
    }

    refract_direction
}
