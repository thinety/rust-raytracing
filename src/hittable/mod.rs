mod sphere;

use crate::material::Material;
use crate::math::{Ray, Vector};

pub use sphere::Sphere;

pub struct HitRecord<'a> {
    pub t: f64,
    pub normal: Vector,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (**self).hit(ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_closest_so_far = t_max;
        let mut option = None;

        for hittable in self.iter() {
            if let Some(hit_record) = hittable.hit(ray, t_min, t_closest_so_far) {
                t_closest_so_far = hit_record.t;
                option = Some(hit_record);
            }
        }

        option
    }
}
