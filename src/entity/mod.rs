mod sphere;

use enum_dispatch::enum_dispatch;

use crate::material::Material;
use crate::math::{Ray, Vector};

pub use sphere::Sphere;

#[enum_dispatch]
pub enum Entity {
    Sphere,
}

#[enum_dispatch(Entity)]
pub trait EntityInterface {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub t: f64,
    pub normal: Vector,
    pub material: &'a Material,
}
