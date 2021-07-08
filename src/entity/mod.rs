mod sphere;

use crate::material::Material;
use crate::math::{Ray, Vector};

pub use sphere::Sphere;

pub struct HitRecord<'a> {
    pub t: f64,
    pub normal: Vector,
    pub material: &'a dyn Material,
}

pub trait Entity {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T> Entity for T
where
    T: std::ops::Deref,
    <T as std::ops::Deref>::Target: Entity
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (**self).hit(ray, t_min, t_max)
    }
}
